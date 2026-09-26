require "test_helper"
require "tmpdir"

class ProductTest < ActionDispatch::IntegrationTest
  include ActiveJob::TestHelper

  PNG_A = "\x89PNG\r\n\x1a\nA".b
  PNG_B = "\x89PNG\r\n\x1a\nB".b

  setup do
    @dir = Dir.mktmpdir
    @env = ENV.to_h.slice("AIRLOCK_GIT_ROOT", "AIRLOCK_WORK_ROOT", "AIRLOCK_REVIEW_TOKEN")
    ENV["AIRLOCK_GIT_ROOT"] = @dir
    ENV["AIRLOCK_WORK_ROOT"] = File.join(@dir, "work")
    ENV["AIRLOCK_REVIEW_TOKEN"] = "rev-token"
    bare = File.join(@dir, "frameline.git")
    work = File.join(@dir, "seed")
    sh "git init -q --bare -b main #{bare} && git init -q -b main #{work}"
    write(work, "tests/golden/lumen_ad.png", PNG_A)
    write(work, "tests/golden/tortoise_story.png", PNG_A)
    git work, "add -A && git commit -qm seed"
    @root = `git -C #{work} rev-parse HEAD`.strip
    write(work, "tests/golden/lumen_ad.png", PNG_B)
    git work, "commit -qam 'Add a sway motion'"
    git work, "push -q #{bare} main"
    @main = `git -C #{work} rev-parse HEAD`.strip
    # A branch waiting for review that proposes a new board.
    git work, "checkout -qb review"
    write(work, "tests/golden/cafe_morning.png", PNG_B)
    git work, "add -A && git commit -qm 'Add a cafe example' && git push -q #{bare} review:refs/heads/agents/agent-lin/fl-2"
    @review_sha = `git -C #{work} rev-parse HEAD`.strip

    decision = GateDecision.create!(repo: "frameline", ref: "refs/heads/agents/agent-lin/fl-2", pusher: "agent-lin", verdict: "accept",
                                    changed_paths: ["tests/golden/cafe_morning.png", "examples/specs/cafe_morning.yaml"])
    @review = Change.create!(repo: "frameline", ref: decision.ref, pusher: "agent-lin", head_sha: @review_sha,
                             gate_decision: decision, state: "needs_review", review_reasons: ["sensitive path tests/golden/cafe_morning.png"])
    merged = GateDecision.create!(repo: "frameline", ref: "refs/heads/agents/agent-kai/fl-1", pusher: "agent-kai", verdict: "accept")
    Change.create!(repo: "frameline", ref: merged.ref, pusher: "agent-kai", head_sha: @main, gate_decision: merged, state: "merged")
    AgentRun.create!(repo: "frameline", agent: "agent-ada", task: "FL-3", status: "gave_up", attempts: 6)
  end

  teardown do
    %w[AIRLOCK_GIT_ROOT AIRLOCK_WORK_ROOT AIRLOCK_REVIEW_TOKEN].each { |k| @env[k] ? ENV[k] = @env[k] : ENV.delete(k) }
    FileUtils.rm_rf(@dir)
  end

  def sh(cmd) = assert(system(cmd, err: File::NULL), cmd)
  def git(dir, cmd) = sh("cd #{dir} && git -c user.email=t@t -c user.name=t #{cmd}")

  def write(dir, path, bytes)
    FileUtils.mkdir_p(File.dirname(File.join(dir, path)))
    File.binwrite(File.join(dir, path), bytes)
  end

  test "the tab shows the brief, the tasks and the boards before and after" do
    get "/frameline"
    assert_response :ok
    assert_includes response.body, "Lumen desk lamp, 30-second spot"
    assert_includes response.body, "The paper boat in the opening shot should rock gently"

    get "/frameline.json"
    p = response.parsed_body
    tasks = p["tasks"].index_by { |t| t["id"] }
    assert_equal %w[merged review gave\ up not\ started], tasks.values_at("FL-1", "FL-2", "FL-3", "FL-4").map { |t| t["status"] }
    assert_equal ["A1"], tasks["FL-1"]["asks"]
    boards = p["boards"].index_by { |b| b["name"] }
    assert boards["lumen_ad"]["changed"]
    refute boards["tortoise_story"]["changed"]
    assert_equal [@root, @main], boards["lumen_ad"].values_at("before", "after")
    assert_equal ["Add a sway motion"], p["history"].map { |c| c["subject"] }
    review = p["reviews"].find { |r| r["branch"] == "agents/agent-lin/fl-2" }
    assert_equal ["cafe_morning", nil, @review_sha], review["boards"].first.values_at("name", "before", "after")
  end

  test "changes that need a person for other reasons are listed without boards" do
    d = GateDecision.create!(repo: "frameline", ref: "refs/heads/agents/agent-lin/fl-8", pusher: "agent-lin", verdict: "accept",
                             changed_paths: ["src/core/color.rs"])
    Change.create!(repo: "frameline", ref: d.ref, pusher: "agent-lin", head_sha: @main, gate_decision: d, state: "needs_review",
                   review_reasons: ["risk 0.48 above threshold 0.35"])
    get "/frameline.json"
    fl8 = response.parsed_body["reviews"].find { |r| r["branch"].end_with?("fl-8") }
    assert_equal [], fl8["boards"]
    get "/frameline"
    assert_includes response.body, "No board changes"
  end

  test "golden images come from git by commit id only" do
    get "/frameline/golden/#{@main}/lumen_ad"
    assert_response :ok
    assert_equal PNG_B, response.body
    assert_match "immutable", response.headers["Cache-Control"]
    get "/frameline/golden/#{@root}/lumen_ad"
    assert_equal PNG_A, response.body

    get "/frameline/golden/#{@main}/missing"
    assert_response :not_found
    get "/frameline/golden/main/lumen_ad"
    assert_response :not_found
    assert_nil Airlock::RepoFiles.for("frameline").golden_png(@main, "Cargo.toml")
    assert_nil Airlock::RepoFiles.for("frameline").golden_png(@main, "tests/golden/../../Cargo.toml")
  end

  test "renders of main are served by name, the manifest is not" do
    out = File.join(@dir, "out")
    write(out, "lumen_ad.png", PNG_B)
    write(out, "lumen_ad.mp4", "mp4")
    write(out, "notes.txt", "x")
    File.symlink("/etc/hostname", File.join(out, "link.png"))
    manifest = Airlock::Renders.new("frameline").store!(@main, from: out, ok: true, output: "done")
    assert_equal %w[lumen_ad.mp4 lumen_ad.png], manifest["files"]

    get "/frameline/renders/#{@main}/lumen_ad.mp4"
    assert_response :ok
    assert_equal "video/mp4", response.media_type
    get "/frameline/renders/#{@main}/manifest.json"
    assert_response :not_found
    get "/frameline/renders/#{@main}/link.png"
    assert_response :not_found
    get "/frameline"
    assert_includes response.body, "Latest render of"
  end

  test "a reviewer signs in with the token and approves from the tab" do
    post "/frameline/reviews/#{@review.id}/approve"
    assert_redirected_to "/frameline/reviewer/new"
    assert_equal "needs_review", @review.reload.state

    post "/frameline/reviewer", params: { name: "Enrique", token: "wrong" }
    assert_response :unauthorized
    post "/frameline/reviewer", params: { name: "Enrique", token: "rev-token" }
    assert_redirected_to "/frameline"
    get "/frameline"
    assert_includes response.body, "Signed in as Enrique"
    assert_includes response.body, "Approve"

    assert_enqueued_with(job: MergeQueueJob, args: ["frameline"]) do
      post "/frameline/reviews/#{@review.id}/approve"
    end
    assert_equal %w[approved Enrique], @review.reload.slice(:state, :reviewer).values
    post "/frameline/reviews/#{@review.id}/reject", params: { reason: "too late" }
    follow_redirect!
    assert_includes response.body, "change is approved"
  end

  test "a rejection reason is kept and handed to the next agent" do
    Airlock::Review.decide!(@review, state: "rejected", reviewer: "Enrique", reason: "The boats are too small.")
    assert_equal %w[rejected_by_reviewer human agent_retry], @review.reload.diagnosis.values_at("category", "source", "next_step")
    get "/frameline.json"
    fl2 = response.parsed_body["tasks"].find { |t| t["id"] == "FL-2" }
    assert_equal ["rejected", "The boats are too small."], fl2.values_at("status", "detail")

    note = Airlock::Swarm.send(:with_history, "frameline", Airlock::Assignment.new(id: "FL-2", title: "t", instructions: "Do it.", context: [], check: "true"))
    assert_match "A reviewer rejected an earlier attempt", note.instructions
    assert_match "The boats are too small.", note.instructions
  end

  test "a visual change waiting for review shows the proposed animatic once rendered" do
    get "/frameline"
    assert_includes response.body, "rendering this branch"
    out = File.join(@dir, "proposed")
    write(out, "lumen_ad.mp4", "mp4")
    Airlock::Renders.new("frameline").store!(@review_sha, from: out, ok: true, output: "")
    get "/frameline"
    assert_includes response.body, "/frameline/renders/#{@review_sha}/lumen_ad.mp4"
    get "/frameline.json"
    assert_nil response.parsed_body["render"], "a branch render is never shown as main's"
  end

  test "the render job renders main with the policy's command into the store" do
    policy = Airlock::Policy.new("render" => { "command" => "mkdir -p out/airlock && cp tests/golden/lumen_ad.png out/airlock/board.png" })
    Airlock::Policy.stub_load(policy) { RenderJob.perform_now("frameline", @main) }
    manifest = Airlock::Renders.new("frameline").manifest(@main)
    assert manifest["ok"], manifest["output"]
    assert_equal ["board.png"], manifest["files"]
  end
end
