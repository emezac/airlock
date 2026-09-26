require "rubygems/package"
require "zlib"
require "stringio"

module Airlock
  # Brings generated files (golden images) back from a sandbox run. The
  # sandbox prints a base64 tarball between markers; only regular files under
  # the allowed prefixes are extracted, never links or paths outside the tree.
  module Collector
    BEGIN_MARK = "__AIRLOCK_COLLECT_BEGIN__".freeze
    END_MARK = "__AIRLOCK_COLLECT_END__".freeze
    MAX_BYTES = 8 * 1024 * 1024
    Invalid = Class.new(StandardError)

    module_function

    def script(paths)
      list = paths.map { |p| "'#{p.delete("'")}'" }.join(" ")
      "echo #{BEGIN_MARK}; tar -czf - #{list} | base64 | tr -d '\\n'; echo; echo #{END_MARK}"
    end

    # Returns [output without the payload, extracted paths].
    def extract!(output, dir, allowed)
      payload = output[/#{BEGIN_MARK}\n(.*?)\n#{END_MARK}/m, 1]
      clean = output.sub(/#{BEGIN_MARK}.*?#{END_MARK}\n?/m, "")
      return [clean, []] unless payload

      bytes = payload.unpack1("m")
      raise Invalid, "collected files exceed #{MAX_BYTES} bytes" if bytes.bytesize > MAX_BYTES

      [clean, untar!(bytes, dir, allowed)]
    end

    def untar!(bytes, dir, allowed)
      root = File.realpath(dir)
      written = []
      Gem::Package::TarReader.new(Zlib::GzipReader.new(StringIO.new(bytes))) do |tar|
        tar.each do |entry|
          next if entry.directory?
          raise Invalid, "#{entry.full_name}: only regular files are collected" unless entry.file?

          name = Pathname(entry.full_name).cleanpath.to_s
          unless allowed.any? { |p| name.start_with?(p) } && !name.start_with?("/", "..")
            raise Invalid, "#{name}: outside the collected paths"
          end

          target = File.join(root, name)
          FileUtils.mkdir_p(File.dirname(target))
          raise Invalid, "#{name}: path goes through a symlink" unless File.realpath(File.dirname(target)).start_with?(root)

          File.binwrite(target, entry.read.to_s)
          written << name
        end
      end
      written
    end
  end
end
