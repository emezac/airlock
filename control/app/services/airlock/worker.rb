module Airlock
  # An agent that takes one Assignment from idea to a pushed branch:
  #
  #   1. branch from main in its own clone
  #   2. ask the model for edits (Edits JSON), apply them
  #   3. run the check in the runner (a Token Factory Sandbox in production);
  #      on failure, show the model the output and try again
  #   4. squash to one commit whose trailer cites the passing run
  #   5. push through the gate; if the gate refuses, show the model why
  #
  # The model never runs anything and never touches git: it only proposes
  # edits. Commands come from the assignment and the policy.
  class Worker
    Result = Data.define(:status, :agent, :assignment, :branch, :sha, :attempts, :input_tokens, :output_tokens,
                         :gate_output, :log) do
      def pushed? = status == "pushed"
    end

    GOLDEN_PATHS = ["tests/golden/"].freeze
    UPDATE_GOLDENS = "UPDATE_GOLDEN=1 cargo test --release --test visual".freeze

    def initialize(agent:, workspace:, runner:, model:, policy: Policy.load, max_attempts: 6,
                   model_name: TokenFactory::SUPER, update_goldens_command: UPDATE_GOLDENS, thinking: true)
      @agent = agent
      @workspace = workspace
      @runner = runner
      @model = model
      @policy = policy
      @max_attempts = max_attempts
      @model_name = model_name
      @update_goldens_command = update_goldens_command
      # Measured on the Frameline backlog: without reasoning, Nemotron Super kept the
      # SEARCH/REPLACE format in the first block and lost it later; with it, replies parsed.
      @thinking = thinking
    end

    # on_note: called with each progress line, for live status.
    def run(assignment, on_note: nil)
      @on_note = on_note
      @log = []
      @last_paths = []
      @tokens = [0, 0]
      branch = "agents/#{@agent}/#{assignment.slug}"
      @workspace.prepare!
      @workspace.start_branch(branch)
      messages = [{ role: "system", content: system_prompt }, { role: "user", content: task_prompt(assignment) }]
      touched = []

      1.upto(@max_attempts) do |attempt|
        progress "attempt #{attempt}: waiting for #{@model_name.split('/').last}"
        reply = ask(messages)
        messages << { role: "assistant", content: reply }
        @last_paths = reply.scan(/^(\S+)\n<{5,9} SEARCH/).flatten.uniq
        edits = Edits.parse(reply)
        touched |= edits.apply!(@workspace.path)
        note "attempt #{attempt}: #{edits.summary} (#{touched.join(', ')})"
        if edits.update_goldens?
          regen = @runner.call(@workspace.path, command: @update_goldens_command, collect: GOLDEN_PATHS)
          note "regenerated golden images: #{regen.ok ? 'ok' : 'failed'}"
        end
        @workspace.commit_all("wip #{assignment.id} attempt #{attempt}")
        progress "attempt #{attempt}: running #{assignment.check}"
        check = @runner.call(@workspace.path, command: assignment.check)
        unless check.ok
          note "check failed: #{assignment.check}"
          messages << { role: "user", content: failure_prompt(assignment, check.output, touched) }
          next
        end

        sha = @workspace.squash(commit_message(assignment, edits, check))
        accepted, output = @workspace.push_branch(branch, @agent)
        gate = gate_lines(output)
        note "gate: #{accepted ? 'accepted' : 'refused'} #{gate}"
        return result("pushed", assignment, branch, sha, attempt, gate) if accepted

        messages << { role: "user", content: gate_prompt(gate) }
      rescue Edits::Invalid => e
        note "attempt #{attempt}: edits refused: #{e.message}"
        messages << { role: "user", content: "Your edits were not applied: #{e.message}. Nothing changed. " \
                                             "Reply again in the same format, with SEARCH copied from the files as they are now:\n\n" \
                                             "#{files_block((touched | attempted_paths(e)))}" }
      end
      result("gave_up", assignment, branch, nil, @max_attempts, nil)
    end

    private

    def ask(messages)
      reply = @model.chat(model: @model_name, messages: messages, max_tokens: @thinking ? 16_000 : 6000, temperature: 0.2,
                           json: false, thinking: @thinking)
      @tokens[0] += reply.input_tokens
      @tokens[1] += reply.output_tokens
      reply.content
    end

    def note(line)
      @log << line
      @on_note&.call(line)
    end

    # Live status only; not part of the result's log.
    def progress(line) = @on_note&.call(line)

    def result(status, assignment, branch, sha, attempts, gate)
      Result.new(status: status, agent: @agent, assignment: assignment.id, branch: branch, sha: sha, attempts: attempts,
                 input_tokens: @tokens[0], output_tokens: @tokens[1], gate_output: gate, log: @log)
    end

    def commit_message(assignment, edits, check)
      evidence = %(sandbox=#{check.run_id || 'local'} checkpoint=#{check.checkpoint || 'none'} cmd="#{assignment.check}")
      "#{assignment.title}\n\n#{edits.summary}\n\nAirlock-Task: #{assignment.id}\n#{@policy.evidence_trailer}: #{evidence}\n"
    end

    def gate_lines(output)
      output.lines.map { |l| l.sub(/\Aremote:\s*/, "").strip }.select { |l| l.start_with?("airlock:") }.join(" ")
    end

    def system_prompt
      <<~PROMPT
        You are #{@agent}, a software agent working on Frameline, a Rust crate that renders storyboards
        and animatics from YAML specs (src/spec.rs, src/board.rs, src/bin/frameline.rs) on top of the
        taller_film renderer. Your work is pushed through Airlock, a gate that checks every change.

        Reply in exactly this format, with no other text:

        SUMMARY: one or two sentences on what you changed and why
        UPDATE_GOLDENS: no

        src/spec.rs
        <<<<<<< SEARCH
        exact lines copied from the current file
        =======
        the lines that replace them
        >>>>>>> REPLACE

        Rules:
        - Write code exactly as it appears in the file. Nothing is escaped: no JSON, no quoting.
        - The file path goes alone on the line before <<<<<<< SEARCH.
        - SEARCH must match the current file exactly, including indentation, and match only once.
          Use the 2 to 10 lines around the change. To add code, SEARCH for a nearby line and put
          that same line plus the new code in REPLACE.
        - Use as many blocks as you need, one per change. To create a file, leave SEARCH empty.
        - Do the whole task: change the implementation and add or update the tests that prove it.
        - Never delete, skip or weaken a test, and never edit tests/golden/ yourself.
        - Do not add dependencies or edit Cargo.toml.
        - Set UPDATE_GOLDENS: yes if your change alters how an example board looks or adds a file
          to examples/specs/. The golden images are regenerated for you and a human reviews them.
        - Captions and labels may use only letters, digits, spaces and . : - / ° ( ) , ' ! ?
      PROMPT
    end

    def task_prompt(assignment)
      <<~PROMPT
        Task #{assignment.id}: #{assignment.title}

        #{assignment.instructions.strip}

        Your change must pass `#{assignment.check}`.

        Current files:

        #{files_block(assignment.context)}
      PROMPT
    end

    def failure_prompt(assignment, output, touched)
      <<~PROMPT
        `#{assignment.check}` failed. The errors and failing tests:

        ```
        #{FailureDigest.call(output)}
        ```

        Your earlier edits are still applied. Reply with new edits against the current files:

        #{files_block(touched)}
      PROMPT
    end

    # Files the refused reply tried to touch, so the model sees them as they are.
    def attempted_paths(_error) = @last_paths.to_a

    def gate_prompt(gate)
      "Airlock refused the push: #{gate}\nFix the cause with new edits. Do not work around the gate."
    end

    def files_block(paths)
      root = File.realpath(@workspace.path)
      paths.filter_map do |path|
        file = File.join(root, path)
        next "### #{path}\n(does not exist yet)" unless File.file?(file)
        next if Pathname(path).absolute? || path.split("/").include?(".git") || !File.realpath(file).start_with?("#{root}/")

        "### #{path}\n```\n#{File.read(file)}\n```"
      end.join("\n\n")
    end
  end
end
