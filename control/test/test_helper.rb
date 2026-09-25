ENV["RAILS_ENV"] ||= "test"
require_relative "../config/environment"
require "rails/test_help"

module ActiveSupport
  class TestCase
    # Run tests in parallel with specified workers
    parallelize(workers: :number_of_processors)

    # Setup all fixtures in test/fixtures/*.yml for all tests in alphabetical order.
    fixtures :all

    # Add more helper methods to be used by all tests here...
  end
end

# Tests never call Token Factory: the labeler runs with rules only.
class Airlock::Labeler
  def self.stub_any_instance_model_free
    original = instance_method(:model_labels)
    define_method(:model_labels) { |_push| [] }
    yield
  ensure
    define_method(:model_labels, original)
  end
end
