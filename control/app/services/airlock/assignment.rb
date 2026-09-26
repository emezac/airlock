require "yaml"

module Airlock
  # One unit of work for an agent: what to do, which files to read first and
  # the command whose passing run becomes the commit's evidence.
  class Assignment < Data.define(:id, :title, :instructions, :context, :check)
    # A backlog may list shared context (a crate map) that every task reads first.
    def self.load_all(path)
      data = YAML.safe_load_file(path)
      shared = Array(data["context"]).map(&:to_s)
      data.fetch("tasks").map { |t| from_h(t, shared: shared) }
    end

    def self.from_h(h, shared: [])
      new(id: h.fetch("id").to_s, title: h.fetch("title").to_s, instructions: h.fetch("instructions").to_s,
          context: (shared + Array(h["context"]).map(&:to_s)).uniq, check: h.fetch("check", "cargo test --release").to_s)
    end

    def slug = id.downcase.gsub(/[^a-z0-9]+/, "-").gsub(/\A-|-\z/, "")
  end
end
