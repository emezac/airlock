module Airlock
  # Human attention as a server with capacity mu reviews per hour. Changes
  # arrive at lambda per hour and a fraction P(risk > t) goes to a human, so
  #   rho = lambda * P(risk > t) / mu.
  # Waiting time grows like 1 / (mu - lambda_h) and explodes as rho -> 1, so
  # we pick the smallest threshold t that keeps rho <= target.
  class Attention
    WINDOW = 2.hours
    MIN_REVIEW_SAMPLES = 5

    Estimate = Data.define(:lambda_per_hour, :mu_per_hour, :threshold, :expected_rho, :mu_source)

    def initialize(policy, now: Time.current)
      @policy = policy
      @now = now
    end

    def estimate(repo)
      recent = Change.where(repo: repo, created_at: (@now - WINDOW)..@now).where.not(risk_score: nil)
      lam = recent.count / (WINDOW / 1.hour.to_f)
      mu, source = service_rate(repo)
      t = self.class.threshold(recent.pluck(:risk_score), lam, mu, @policy.target_utilization, @policy.min_threshold)
      share = recent.empty? ? 0.0 : recent.count { |c| c.risk_score > t } / recent.size.to_f
      Estimate.new(lambda_per_hour: lam.round(2), mu_per_hour: mu.round(2), threshold: t.round(3),
                   expected_rho: (mu.positive? ? lam * share / mu : 0.0).round(3), mu_source: source)
    end

    # Largest share of changes humans can take: s = target * mu / lambda.
    # The threshold is the (1 - s) quantile of recent risk scores, never below
    # min_threshold, so load can raise the bar but cannot lower it.
    def self.threshold(scores, lam, mu, target, min_threshold)
      return min_threshold if scores.empty? || lam <= 0

      share = (target * mu / lam).clamp(0.0, 1.0)
      return min_threshold if share >= 1.0

      sorted = scores.sort
      index = ((1.0 - share) * (sorted.size - 1)).ceil
      [sorted[index], min_threshold].max
    end

    private

    def service_rate(repo)
      durations = Change.where(repo: repo).where.not(reviewed_at: nil).where.not(review_requested_at: nil)
                        .order(reviewed_at: :desc).limit(50)
                        .pluck(:review_requested_at, :reviewed_at).map { |a, b| (b - a).to_f }
      return [@policy.reviews_per_hour, "prior"] if durations.size < MIN_REVIEW_SAMPLES

      mean = durations.sum / durations.size
      [3600.0 / [mean, 1.0].max, "measured"]
    end
  end
end
