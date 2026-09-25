module Airlock
  # Parses `Airlock-Evidence: sandbox=<id> checkpoint=<id> cmd="<command>"`.
  class Evidence < Data.define(:sandbox, :checkpoint, :cmd)
    REQUIRED = %i[sandbox checkpoint cmd].freeze

    def self.parse(value)
      return nil if value.nil?

      fields = value.scan(/(\w+)=("([^"]*)"|\S+)/).to_h { |key, raw, quoted| [key.to_sym, quoted || raw] }
      return nil unless REQUIRED.all? { |key| fields[key].to_s.strip != "" }

      new(**fields.slice(*REQUIRED))
    end
  end
end
