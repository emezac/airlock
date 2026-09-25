# This file is auto-generated from the current state of the database. Instead
# of editing this file, please use the migrations feature of Active Record to
# incrementally modify your database, and then regenerate this schema definition.
#
# This file is the source Rails uses to define your schema when running `bin/rails
# db:schema:load`. When creating a new database, `bin/rails db:schema:load` tends to
# be faster and is potentially less error prone than running all of your
# migrations from scratch. Old migrations may fail to apply correctly if those
# migrations use external dependencies or application code.
#
# It's strongly recommended that you check this file into your version control system.

ActiveRecord::Schema[8.1].define(version: 2026_09_25_215311) do
  # These are extensions that must be enabled in order to support this database
  enable_extension "pg_catalog.plpgsql"
  enable_extension "pg_trgm"
  enable_extension "pgcrypto"

  create_table "agent_stats", force: :cascade do |t|
    t.string "agent"
    t.float "failure_rate", default: 0.1, null: false
    t.integer "observations", default: 0, null: false
    t.datetime "created_at", null: false
    t.datetime "updated_at", null: false
    t.index ["agent"], name: "index_agent_stats_on_agent", unique: true
  end

  create_table "agentkit_a2a_tasks", force: :cascade do |t|
    t.string "tenant_key", default: "__global__", null: false
    t.string "task_id", null: false
    t.jsonb "payload", default: {}, null: false
    t.datetime "created_at", null: false
    t.datetime "updated_at", null: false
    t.index ["tenant_key", "created_at"], name: "index_agentkit_a2a_tasks_on_tenant_key_and_created_at"
    t.index ["tenant_key", "task_id"], name: "idx_agentkit_a2a_tasks_tenant_id", unique: true
  end

  create_table "agentkit_action_decisions", force: :cascade do |t|
    t.bigint "proposal_id", null: false
    t.string "tenant_key", default: "__global__", null: false
    t.string "decision", null: false
    t.string "actor_principal_id", null: false
    t.datetime "decided_at", null: false
    t.string "approved_arguments_digest"
    t.string "policy_version", null: false
    t.string "reason_code"
    t.datetime "created_at", null: false
    t.datetime "updated_at", null: false
    t.index ["proposal_id"], name: "index_agentkit_action_decisions_on_proposal_id", unique: true
    t.index ["tenant_key", "decided_at"], name: "index_agentkit_action_decisions_on_tenant_key_and_decided_at"
  end

  create_table "agentkit_action_outboxes", force: :cascade do |t|
    t.bigint "proposal_id", null: false
    t.string "tenant_key", default: "__global__", null: false
    t.string "event_type", default: "action.execute", null: false
    t.string "status", default: "pending", null: false
    t.integer "delivery_attempts", default: 0, null: false
    t.datetime "dispatched_at"
    t.string "last_error_code"
    t.datetime "created_at", null: false
    t.datetime "updated_at", null: false
    t.index ["proposal_id"], name: "index_agentkit_action_outboxes_on_proposal_id", unique: true
    t.index ["status", "created_at"], name: "index_agentkit_action_outboxes_on_status_and_created_at"
  end

  create_table "agentkit_action_outcomes", force: :cascade do |t|
    t.bigint "proposal_id", null: false
    t.string "tenant_key", default: "__global__", null: false
    t.string "kind", null: false
    t.datetime "observed_at", null: false
    t.jsonb "value"
    t.string "evidence_digest", null: false
    t.string "source", null: false
    t.float "confidence"
    t.datetime "created_at", null: false
    t.datetime "updated_at", null: false
    t.index ["proposal_id", "observed_at"], name: "index_agentkit_action_outcomes_on_proposal_id_and_observed_at"
    t.index ["proposal_id"], name: "index_agentkit_action_outcomes_on_proposal_id"
  end

  create_table "agentkit_action_proposals", force: :cascade do |t|
    t.string "public_id", null: false
    t.string "tenant_key", default: "__global__", null: false
    t.string "action_type", null: false
    t.jsonb "arguments", default: {}, null: false
    t.string "arguments_digest", null: false
    t.string "requester_principal_id", null: false
    t.string "required_permission"
    t.string "target_type"
    t.string "target_id"
    t.string "risk", null: false
    t.string "effect", null: false
    t.string "policy_version", null: false
    t.string "contract_version", null: false
    t.string "status", default: "draft", null: false
    t.string "operation_namespace", null: false
    t.string "idempotency_key"
    t.string "source_adapter"
    t.jsonb "canonical_response"
    t.datetime "expires_at"
    t.integer "lock_version", default: 0, null: false
    t.datetime "created_at", null: false
    t.datetime "updated_at", null: false
    t.index ["public_id"], name: "index_agentkit_action_proposals_on_public_id", unique: true
    t.index ["tenant_key", "operation_namespace", "idempotency_key"], name: "idx_agentkit_actions_durable_idempotency", unique: true, where: "(idempotency_key IS NOT NULL)"
    t.index ["tenant_key", "status", "updated_at"], name: "idx_agentkit_actions_lifecycle"
  end

  create_table "agentkit_artifacts", force: :cascade do |t|
    t.bigint "run_id"
    t.string "kind"
    t.string "content_type"
    t.text "body"
    t.jsonb "metadata", default: {}, null: false
    t.string "content_hash"
    t.string "tenant_key", default: "__global__", null: false
    t.datetime "created_at", null: false
    t.datetime "updated_at", null: false
    t.index ["content_hash"], name: "index_agentkit_artifacts_on_content_hash"
    t.index ["run_id"], name: "index_agentkit_artifacts_on_run_id"
    t.index ["tenant_key"], name: "index_agentkit_artifacts_on_tenant_key"
  end

  create_table "agentkit_asset_bindings", force: :cascade do |t|
    t.string "tenant_key", default: "__global__", null: false
    t.bigint "account_id"
    t.bigint "asset_id", null: false
    t.string "agent_name", null: false
    t.string "target_type"
    t.bigint "target_id"
    t.integer "priority", default: 50, null: false
    t.datetime "created_at", null: false
    t.datetime "updated_at", null: false
    t.index ["account_id"], name: "index_agentkit_asset_bindings_on_account_id"
    t.index ["asset_id", "agent_name"], name: "index_agentkit_asset_bindings_on_asset_id_and_agent_name", unique: true
    t.index ["asset_id"], name: "index_agentkit_asset_bindings_on_asset_id"
    t.index ["tenant_key"], name: "index_agentkit_asset_bindings_on_tenant_key"
  end

  create_table "agentkit_audit_chain_heads", force: :cascade do |t|
    t.string "tenant_key", null: false
    t.bigint "sequence", default: 0, null: false
    t.string "last_hash"
    t.datetime "created_at", null: false
    t.datetime "updated_at", null: false
    t.index ["tenant_key"], name: "index_agentkit_audit_chain_heads_on_tenant_key", unique: true
  end

  create_table "agentkit_audit_logs", force: :cascade do |t|
    t.string "event_type", null: false
    t.string "agent_name"
    t.string "status"
    t.text "prompt_preview"
    t.string "model"
    t.integer "input_tokens"
    t.integer "output_tokens"
    t.decimal "cost_usd", precision: 12, scale: 6
    t.integer "duration_ms"
    t.jsonb "payload", default: {}, null: false
    t.uuid "trace_id"
    t.uuid "run_id"
    t.string "step_key"
    t.bigint "user_id"
    t.bigint "account_id"
    t.string "tenant_key", default: "__global__", null: false
    t.string "subject_type"
    t.bigint "subject_id"
    t.datetime "occurred_at", null: false
    t.datetime "created_at", null: false
    t.datetime "updated_at", null: false
    t.integer "schema_version", default: 1, null: false
    t.bigint "sequence"
    t.string "principal_id"
    t.string "payload_digest"
    t.string "previous_hash"
    t.string "event_hash"
    t.string "signature"
    t.string "key_id"
    t.index ["agent_name", "occurred_at"], name: "index_agentkit_audit_logs_on_agent_name_and_occurred_at"
    t.index ["event_hash"], name: "idx_agentkit_audit_event_hash", unique: true, where: "(event_hash IS NOT NULL)"
    t.index ["event_type", "occurred_at"], name: "index_agentkit_audit_logs_on_event_type_and_occurred_at"
    t.index ["run_id"], name: "index_agentkit_audit_logs_on_run_id"
    t.index ["subject_type", "subject_id"], name: "index_agentkit_audit_logs_on_subject_type_and_subject_id"
    t.index ["tenant_key", "occurred_at"], name: "index_agentkit_audit_logs_on_tenant_key_and_occurred_at"
    t.index ["tenant_key", "sequence"], name: "idx_agentkit_audit_chain_sequence", unique: true, where: "(schema_version = 2)"
    t.index ["tenant_key"], name: "index_agentkit_audit_logs_on_tenant_key"
    t.index ["trace_id"], name: "index_agentkit_audit_logs_on_trace_id"
  end

  create_table "agentkit_code_symbols", force: :cascade do |t|
    t.string "tenant_key", default: "__global__", null: false
    t.bigint "account_id"
    t.bigint "asset_id", null: false
    t.string "name", null: false
    t.string "symbol_type", null: false
    t.string "file_path", null: false
    t.integer "line_number"
    t.jsonb "callers", default: [], null: false
    t.jsonb "callees", default: [], null: false
    t.datetime "created_at", null: false
    t.datetime "updated_at", null: false
    t.string "qualified_name"
    t.string "file_digest"
    t.jsonb "provenance", default: {}, null: false
    t.decimal "confidence", precision: 5, scale: 4, default: "1.0", null: false
    t.index ["account_id"], name: "index_agentkit_code_symbols_on_account_id"
    t.index ["asset_id", "file_path"], name: "index_agentkit_code_symbols_on_asset_id_and_file_path"
    t.index ["asset_id", "name"], name: "index_agentkit_code_symbols_on_asset_id_and_name"
    t.index ["asset_id"], name: "index_agentkit_code_symbols_on_asset_id"
    t.index ["tenant_key"], name: "index_agentkit_code_symbols_on_tenant_key"
  end

  create_table "agentkit_decisions", force: :cascade do |t|
    t.bigint "suggestion_id"
    t.string "agent_name"
    t.string "suggestion_type"
    t.string "prompt_id"
    t.integer "prompt_version"
    t.string "model"
    t.string "decision", null: false
    t.string "actor", null: false
    t.string "mode", null: false
    t.string "rejection_code"
    t.text "rejection_note"
    t.jsonb "proposed_payload", default: {}, null: false
    t.jsonb "final_payload", default: {}, null: false
    t.float "edit_distance"
    t.integer "time_to_decision_s"
    t.string "outcome"
    t.float "outcome_value"
    t.datetime "outcome_at"
    t.string "tenant_key", default: "__global__", null: false
    t.datetime "created_at", null: false
    t.datetime "updated_at", null: false
    t.bigint "experiment_id"
    t.string "experiment_arm"
    t.index ["agent_name", "created_at"], name: "index_agentkit_decisions_on_agent_name_and_created_at"
    t.index ["experiment_id", "experiment_arm", "mode"], name: "idx_agentkit_decisions_experiment_cohort"
    t.index ["experiment_id"], name: "index_agentkit_decisions_on_experiment_id"
    t.index ["mode", "decision"], name: "index_agentkit_decisions_on_mode_and_decision"
    t.index ["rejection_code"], name: "index_agentkit_decisions_on_rejection_code"
    t.index ["suggestion_id"], name: "index_agentkit_decisions_on_suggestion_id"
    t.index ["tenant_key"], name: "index_agentkit_decisions_on_tenant_key"
  end

  create_table "agentkit_events", force: :cascade do |t|
    t.string "name", null: false
    t.uuid "run_id"
    t.uuid "trace_id"
    t.jsonb "dims", default: {}, null: false
    t.jsonb "measures", default: {}, null: false
    t.bigint "account_id"
    t.bigint "user_id"
    t.datetime "occurred_at", null: false
    t.datetime "created_at", null: false
    t.datetime "updated_at", null: false
    t.index ["dims"], name: "index_agentkit_events_on_dims", using: :gin
    t.index ["name", "occurred_at"], name: "index_agentkit_events_on_name_and_occurred_at"
    t.index ["run_id"], name: "index_agentkit_events_on_run_id"
  end

  create_table "agentkit_execution_attempts", force: :cascade do |t|
    t.bigint "proposal_id", null: false
    t.string "tenant_key", default: "__global__", null: false
    t.integer "attempt_number", null: false
    t.string "idempotency_key", null: false
    t.string "status", null: false
    t.datetime "started_at", null: false
    t.datetime "finished_at"
    t.string "error_code"
    t.string "external_result_ref"
    t.jsonb "canonical_response"
    t.datetime "created_at", null: false
    t.datetime "updated_at", null: false
    t.index ["proposal_id", "attempt_number"], name: "idx_agentkit_execution_attempt_number", unique: true
    t.index ["tenant_key", "status", "started_at"], name: "idx_agentkit_execution_lifecycle"
  end

  create_table "agentkit_experiments", force: :cascade do |t|
    t.string "name", null: false
    t.string "level", null: false
    t.string "target", null: false
    t.jsonb "control", default: {}, null: false
    t.jsonb "variant", default: {}, null: false
    t.float "traffic_pct", default: 10.0, null: false
    t.string "bucket_by", default: "account", null: false
    t.jsonb "guardrails", default: {}, null: false
    t.string "status", default: "draft", null: false
    t.jsonb "results", default: {}, null: false
    t.bigint "finding_id"
    t.datetime "started_at"
    t.datetime "finished_at"
    t.datetime "created_at", null: false
    t.datetime "updated_at", null: false
    t.jsonb "cohort", default: {}, null: false
    t.datetime "last_evaluated_at"
    t.integer "lock_version", default: 0, null: false
    t.string "tenant_key", default: "__global__", null: false
    t.bigint "account_id"
    t.index ["account_id"], name: "index_agentkit_experiments_on_account_id"
    t.index ["finding_id"], name: "index_agentkit_experiments_on_finding_id"
    t.index ["target", "status"], name: "index_agentkit_experiments_on_target_and_status"
    t.index ["tenant_key", "target"], name: "idx_agentkit_tenant_running_target", unique: true, where: "((status)::text = 'running'::text)"
    t.index ["tenant_key"], name: "index_agentkit_experiments_on_tenant_key"
  end

  create_table "agentkit_exploration_attempts", force: :cascade do |t|
    t.string "attempt_id", null: false
    t.string "world_id", null: false
    t.string "tenant_key", default: "__global__", null: false
    t.bigint "account_id"
    t.integer "round_number", null: false
    t.integer "position", null: false
    t.string "parent_node_id", null: false
    t.string "idempotency_key", null: false
    t.string "status", default: "pending", null: false
    t.string "outcome_status"
    t.string "node_id"
    t.decimal "score", precision: 20, scale: 8
    t.string "artifact_digest"
    t.string "failure_class"
    t.jsonb "diagnostics", default: {}, null: false
    t.jsonb "metadata", default: {}, null: false
    t.datetime "started_at"
    t.datetime "completed_at"
    t.datetime "created_at", null: false
    t.datetime "updated_at", null: false
    t.index ["attempt_id"], name: "index_agentkit_exploration_attempts_on_attempt_id", unique: true
    t.index ["tenant_key", "idempotency_key"], name: "idx_agentkit_exploration_attempts_idempotency", unique: true
    t.index ["tenant_key", "status", "updated_at"], name: "idx_agentkit_exploration_attempts_status"
    t.index ["tenant_key", "world_id", "round_number", "position"], name: "idx_agentkit_exploration_attempts_slot", unique: true
    t.check_constraint "round_number >= 0 AND \"position\" >= 0", name: "chk_agentkit_exploration_attempt_counts"
  end

  create_table "agentkit_exploration_policy_bindings", force: :cascade do |t|
    t.string "target", null: false
    t.string "tenant_key", default: "__global__", null: false
    t.bigint "account_id", default: 0, null: false
    t.string "policy_name", null: false
    t.string "policy_version", null: false
    t.string "policy_digest", null: false
    t.string "dossier_id", null: false
    t.integer "generation", default: 1, null: false
    t.datetime "applied_at", null: false
    t.integer "lock_version", default: 0, null: false
    t.datetime "created_at", null: false
    t.datetime "updated_at", null: false
    t.index ["tenant_key", "account_id", "target"], name: "idx_agentkit_exploration_policy_binding", unique: true
    t.check_constraint "generation > 0", name: "chk_agentkit_exploration_binding_generation"
  end

  create_table "agentkit_exploration_quota_reservations", force: :cascade do |t|
    t.string "tenant_key", default: "__global__", null: false
    t.bigint "account_id", default: 0, null: false
    t.string "resource", null: false
    t.string "reservation_key", null: false
    t.integer "amount", null: false
    t.date "period_start", null: false
    t.datetime "created_at", null: false
    t.datetime "updated_at", null: false
    t.index ["tenant_key", "account_id", "resource", "reservation_key"], name: "idx_agentkit_exploration_quota_reservation", unique: true
    t.index ["tenant_key", "period_start", "resource"], name: "idx_agentkit_exploration_quota_period"
    t.check_constraint "amount > 0", name: "chk_agentkit_exploration_quota_amount"
  end

  create_table "agentkit_exploration_quota_usages", force: :cascade do |t|
    t.string "tenant_key", default: "__global__", null: false
    t.bigint "account_id", default: 0, null: false
    t.string "resource", null: false
    t.date "period_start", null: false
    t.integer "used", default: 0, null: false
    t.datetime "created_at", null: false
    t.datetime "updated_at", null: false
    t.index ["tenant_key", "account_id", "resource", "period_start"], name: "idx_agentkit_exploration_quota_usage", unique: true
    t.check_constraint "used >= 0", name: "chk_agentkit_exploration_quota_used"
  end

  create_table "agentkit_exploration_reviews", force: :cascade do |t|
    t.string "dossier_id", null: false
    t.string "target", null: false
    t.string "tenant_key", default: "__global__", null: false
    t.bigint "account_id", default: 0, null: false
    t.string "status", default: "pending", null: false
    t.string "incumbent_name", null: false
    t.string "incumbent_version", null: false
    t.string "incumbent_digest", null: false
    t.string "candidate_name", null: false
    t.string "candidate_version", null: false
    t.string "candidate_digest", null: false
    t.string "evidence_digest", null: false
    t.jsonb "evidence", default: {}, null: false
    t.jsonb "previous_binding", default: {}, null: false
    t.string "reviewed_by"
    t.text "review_reason"
    t.datetime "submitted_at", null: false
    t.datetime "decided_at"
    t.integer "lock_version", default: 0, null: false
    t.datetime "created_at", null: false
    t.datetime "updated_at", null: false
    t.index ["dossier_id"], name: "index_agentkit_exploration_reviews_on_dossier_id", unique: true
    t.index ["tenant_key", "account_id", "status", "submitted_at"], name: "idx_agentkit_exploration_review_queue"
    t.index ["tenant_key", "account_id", "target", "evidence_digest"], name: "idx_agentkit_exploration_review_evidence", unique: true
    t.check_constraint "status::text = ANY (ARRAY['pending'::character varying, 'approved'::character varying, 'rejected'::character varying, 'rolled_back'::character varying]::text[])", name: "chk_agentkit_exploration_review_status"
  end

  create_table "agentkit_exploration_worlds", force: :cascade do |t|
    t.string "world_id", null: false
    t.string "tenant_key", default: "__global__", null: false
    t.bigint "account_id"
    t.string "objective_digest", null: false
    t.string "policy_name", null: false
    t.string "policy_version", null: false
    t.string "policy_digest", null: false
    t.string "evaluator_digest", null: false
    t.jsonb "bounds", default: {}, null: false
    t.jsonb "tree", default: {}, null: false
    t.integer "rounds", default: 0, null: false
    t.integer "node_count", default: 0, null: false
    t.decimal "best_score", precision: 20, scale: 8
    t.string "status", null: false
    t.string "stop_reason"
    t.datetime "completed_at"
    t.jsonb "metadata", default: {}, null: false
    t.datetime "created_at", null: false
    t.datetime "updated_at", null: false
    t.integer "checkpoint_version", default: 0, null: false
    t.datetime "last_checkpoint_at"
    t.jsonb "evaluator_manifest", default: {}, null: false
    t.string "lease_owner"
    t.datetime "lease_expires_at"
    t.string "generator_digest"
    t.jsonb "generator_manifest", default: {}, null: false
    t.index ["lease_expires_at"], name: "idx_agentkit_exploration_worlds_lease"
    t.index ["tenant_key", "created_at"], name: "idx_agentkit_exploration_worlds_scope"
    t.index ["tenant_key", "policy_name", "policy_version"], name: "idx_agentkit_exploration_worlds_policy"
    t.index ["tenant_key", "status", "created_at"], name: "idx_agentkit_exploration_worlds_operations"
    t.index ["world_id"], name: "index_agentkit_exploration_worlds_on_world_id", unique: true
    t.check_constraint "rounds >= 0 AND node_count >= 0", name: "chk_agentkit_exploration_world_counts"
  end

  create_table "agentkit_factory_runs", force: :cascade do |t|
    t.string "status", default: "running", null: false
    t.integer "window_days", null: false
    t.integer "detector_count", default: 0, null: false
    t.integer "fired_count", default: 0, null: false
    t.integer "created_count", default: 0, null: false
    t.integer "deduplicated_count", default: 0, null: false
    t.integer "experiments_evaluated", default: 0, null: false
    t.jsonb "error_details", default: [], null: false
    t.jsonb "metadata", default: {}, null: false
    t.datetime "started_at", null: false
    t.datetime "finished_at"
    t.datetime "created_at", null: false
    t.datetime "updated_at", null: false
    t.string "tenant_key", default: "__global__", null: false
    t.bigint "account_id"
    t.index ["account_id"], name: "index_agentkit_factory_runs_on_account_id"
    t.index ["status", "started_at"], name: "index_agentkit_factory_runs_on_status_and_started_at"
    t.index ["tenant_key"], name: "index_agentkit_factory_runs_on_tenant_key"
  end

  create_table "agentkit_findings", force: :cascade do |t|
    t.string "detector", null: false
    t.string "severity", default: "medium", null: false
    t.string "subject"
    t.text "summary"
    t.jsonb "evidence", default: {}, null: false
    t.string "suggested_level", default: "n1", null: false
    t.string "status", default: "open", null: false
    t.datetime "created_at", null: false
    t.datetime "updated_at", null: false
    t.string "fingerprint", null: false
    t.integer "occurrence_count", default: 1, null: false
    t.datetime "first_seen_at", null: false
    t.datetime "last_seen_at", null: false
    t.datetime "resolved_at"
    t.string "resolved_by"
    t.text "resolution_reason"
    t.integer "clean_cycles", default: 0, null: false
    t.string "tenant_key", default: "__global__", null: false
    t.bigint "account_id"
    t.index ["account_id"], name: "index_agentkit_findings_on_account_id"
    t.index ["detector", "subject"], name: "index_agentkit_findings_on_detector_and_subject"
    t.index ["status", "severity"], name: "index_agentkit_findings_on_status_and_severity"
    t.index ["tenant_key", "fingerprint"], name: "idx_agentkit_tenant_active_fingerprint", unique: true, where: "((status)::text = ANY ((ARRAY['open'::character varying, 'accepted'::character varying, 'experimenting'::character varying])::text[]))"
    t.index ["tenant_key"], name: "index_agentkit_findings_on_tenant_key"
  end

  create_table "agentkit_golden_cases", force: :cascade do |t|
    t.string "agent_name", null: false
    t.bigint "suggestion_id"
    t.jsonb "input", default: {}, null: false
    t.jsonb "expected", default: {}, null: false
    t.string "label"
    t.string "rejection_code"
    t.datetime "created_at", null: false
    t.datetime "updated_at", null: false
    t.boolean "reviewed", default: false, null: false
    t.string "tenant_key", default: "__global__", null: false
    t.bigint "account_id"
    t.index ["account_id"], name: "index_agentkit_golden_cases_on_account_id"
    t.index ["tenant_key", "agent_name", "suggestion_id"], name: "idx_agentkit_tenant_golden_suggestion", unique: true, where: "(suggestion_id IS NOT NULL)"
    t.index ["tenant_key"], name: "index_agentkit_golden_cases_on_tenant_key"
  end

  create_table "agentkit_graph_edges", force: :cascade do |t|
    t.bigint "snapshot_id", null: false
    t.string "edge_id", null: false
    t.string "tenant_key", default: "__global__", null: false
    t.bigint "account_id"
    t.string "from_node_id", null: false
    t.string "to_node_id", null: false
    t.string "edge_type", null: false
    t.string "direction", default: "outbound", null: false
    t.decimal "weight", precision: 12, scale: 8, default: "1.0", null: false
    t.string "source_digest", null: false
    t.decimal "confidence", precision: 5, scale: 4, default: "1.0", null: false
    t.string "lifecycle_status", default: "active", null: false
    t.jsonb "metadata", default: {}, null: false
    t.datetime "created_at", null: false
    t.datetime "updated_at", null: false
    t.index ["snapshot_id", "edge_id"], name: "idx_agentkit_graph_edges_identity", unique: true
    t.index ["snapshot_id", "from_node_id", "to_node_id"], name: "idx_agentkit_graph_edges_path"
    t.index ["snapshot_id"], name: "index_agentkit_graph_edges_on_snapshot_id"
    t.index ["tenant_key"], name: "index_agentkit_graph_edges_on_tenant_key"
    t.check_constraint "confidence >= 0::numeric AND confidence <= 1::numeric", name: "chk_agentkit_graph_edge_confidence"
    t.check_constraint "weight >= 0::numeric AND weight <= 1000::numeric", name: "chk_agentkit_graph_edge_weight"
  end

  create_table "agentkit_graph_nodes", force: :cascade do |t|
    t.bigint "snapshot_id", null: false
    t.string "node_id", null: false
    t.string "tenant_key", default: "__global__", null: false
    t.bigint "account_id"
    t.bigint "asset_id", null: false
    t.string "node_type", null: false
    t.string "external_ref", null: false
    t.string "label"
    t.string "lifecycle_status", default: "active", null: false
    t.string "visibility_digest", null: false
    t.string "content_digest", null: false
    t.jsonb "metadata", default: {}, null: false
    t.datetime "created_at", null: false
    t.datetime "updated_at", null: false
    t.index ["snapshot_id", "node_id"], name: "idx_agentkit_graph_nodes_identity", unique: true
    t.index ["snapshot_id"], name: "index_agentkit_graph_nodes_on_snapshot_id"
    t.index ["tenant_key", "asset_id", "external_ref"], name: "idx_agentkit_graph_nodes_scope_ref"
  end

  create_table "agentkit_graph_snapshots", force: :cascade do |t|
    t.string "snapshot_id", null: false
    t.string "tenant_key", default: "__global__", null: false
    t.bigint "account_id"
    t.bigint "asset_id", null: false
    t.integer "schema_version", default: 1, null: false
    t.datetime "generated_at", null: false
    t.integer "node_count", default: 0, null: false
    t.integer "edge_count", default: 0, null: false
    t.string "digest", null: false
    t.string "status", default: "validated", null: false
    t.jsonb "diagnostics", default: {}, null: false
    t.jsonb "metadata", default: {}, null: false
    t.datetime "created_at", null: false
    t.datetime "updated_at", null: false
    t.index ["asset_id"], name: "index_agentkit_graph_snapshots_on_asset_id"
    t.index ["snapshot_id"], name: "index_agentkit_graph_snapshots_on_snapshot_id", unique: true
    t.index ["tenant_key", "asset_id", "digest"], name: "idx_agentkit_graph_snapshots_digest", unique: true
    t.index ["tenant_key", "asset_id", "status"], name: "idx_agentkit_graph_snapshots_scope"
  end

  create_table "agentkit_knowledge_chunks", force: :cascade do |t|
    t.string "tenant_key", default: "__global__", null: false
    t.bigint "account_id"
    t.string "corpus_name", null: false
    t.string "chunk_id", null: false
    t.integer "chapter_index"
    t.string "chapter_title"
    t.text "content", null: false
    t.string "source"
    t.integer "chunk_index", default: 0, null: false
    t.jsonb "metadata", default: {}, null: false
    t.jsonb "bm25_doc_freq", default: {}, null: false
    t.datetime "created_at", null: false
    t.datetime "updated_at", null: false
    t.virtual "search_vector", type: :tsvector, as: "to_tsvector('simple'::regconfig, COALESCE(content, ''::text))", stored: true
    t.index ["account_id"], name: "index_agentkit_knowledge_chunks_on_account_id"
    t.index ["content"], name: "index_agentkit_knowledge_chunks_on_content", opclass: :gin_trgm_ops, using: :gin
    t.index ["search_vector"], name: "index_agentkit_knowledge_chunks_on_search_vector", using: :gin
    t.index ["tenant_key", "corpus_name", "chapter_index"], name: "idx_agentkit_knowledge_tenant_chapter"
    t.index ["tenant_key", "corpus_name", "chunk_id"], name: "idx_agentkit_knowledge_tenant_chunk", unique: true
    t.index ["tenant_key"], name: "index_agentkit_knowledge_chunks_on_tenant_key"
  end

  create_table "agentkit_memories", force: :cascade do |t|
    t.text "content", null: false
    t.string "memory_type", default: "observation", null: false
    t.string "status", default: "raw", null: false
    t.float "confidence", default: 0.7, null: false
    t.float "importance", default: 0.5, null: false
    t.jsonb "tags", default: [], null: false
    t.string "role"
    t.string "source_agent"
    t.bigint "user_id"
    t.bigint "account_id"
    t.string "tenant_key"
    t.string "embedding_status", default: "none", null: false
    t.string "embedding_model"
    t.integer "embedding_dims"
    t.string "content_hash"
    t.bigint "duplicate_of_id"
    t.integer "recall_count", default: 0, null: false
    t.datetime "last_recalled_at"
    t.datetime "promoted_at"
    t.bigint "derived_from_memory_id"
    t.bigint "canonical_memory_id"
    t.bigint "superseded_by_id"
    t.string "ontological_type", default: "real", null: false
    t.uuid "run_id"
    t.datetime "expires_at"
    t.jsonb "metadata", default: {}, null: false
    t.datetime "created_at", null: false
    t.datetime "updated_at", null: false
    t.virtual "search_vector", type: :tsvector, as: "to_tsvector('simple'::regconfig, COALESCE(content, ''::text))", stored: true
    t.string "retention_policy", default: "keep", null: false
    t.datetime "pinned_at"
    t.datetime "archived_at"
    t.index ["account_id", "memory_type"], name: "index_agentkit_memories_on_account_id_and_memory_type"
    t.index ["content"], name: "index_agentkit_memories_on_content", opclass: :gin_trgm_ops, using: :gin
    t.index ["content_hash", "tenant_key"], name: "idx_agentkit_memories_dedupe", unique: true, where: "((embedding_status)::text = 'embedded'::text)"
    t.index ["derived_from_memory_id"], name: "index_agentkit_memories_on_derived_from_memory_id"
    t.index ["expires_at"], name: "index_agentkit_memories_on_expires_at"
    t.index ["ontological_type"], name: "index_agentkit_memories_on_ontological_type"
    t.index ["search_vector"], name: "index_agentkit_memories_on_search_vector", using: :gin
    t.index ["superseded_by_id"], name: "index_agentkit_memories_on_superseded_by_id"
    t.index ["tags"], name: "index_agentkit_memories_on_tags", using: :gin
    t.index ["tenant_key", "expires_at"], name: "idx_agentkit_memories_expiration", where: "((expires_at IS NOT NULL) AND ((status)::text = ANY ((ARRAY['raw'::character varying, 'embedded'::character varying, 'consolidated'::character varying])::text[])))"
    t.index ["tenant_key", "pinned_at"], name: "idx_agentkit_memories_pinned", where: "(pinned_at IS NOT NULL)"
    t.index ["tenant_key", "retention_policy"], name: "index_agentkit_memories_on_tenant_key_and_retention_policy"
    t.index ["tenant_key", "status"], name: "index_agentkit_memories_on_tenant_key_and_status"
  end

  create_table "agentkit_memory_assets", force: :cascade do |t|
    t.string "tenant_key", default: "__global__", null: false
    t.bigint "account_id"
    t.bigint "team_id"
    t.string "asset_type", null: false
    t.string "name", null: false
    t.string "visibility", default: "team", null: false
    t.bigint "owner_id"
    t.string "version", default: "1.0.0", null: false
    t.string "status", default: "ready", null: false
    t.integer "usage_count", default: 0, null: false
    t.jsonb "content", default: {}, null: false
    t.jsonb "bindings", default: [], null: false
    t.datetime "created_at", null: false
    t.datetime "updated_at", null: false
    t.index ["account_id"], name: "index_agentkit_memory_assets_on_account_id"
    t.index ["asset_type", "name"], name: "index_agentkit_memory_assets_on_asset_type_and_name"
    t.index ["team_id", "visibility"], name: "index_agentkit_memory_assets_on_team_id_and_visibility"
    t.index ["team_id"], name: "index_agentkit_memory_assets_on_team_id"
    t.index ["tenant_key", "asset_type", "name"], name: "idx_agentkit_assets_tenant_type_name"
    t.index ["tenant_key"], name: "index_agentkit_memory_assets_on_tenant_key"
  end

  create_table "agentkit_metrics", force: :cascade do |t|
    t.string "name", null: false
    t.string "period", default: "day", null: false
    t.date "period_start", null: false
    t.jsonb "dims", default: {}, null: false
    t.integer "count", default: 0, null: false
    t.float "sum"
    t.float "min"
    t.float "max"
    t.float "p50"
    t.float "p90"
    t.float "p95"
    t.float "p99"
    t.float "stddev"
    t.jsonb "histogram", default: {}, null: false
    t.datetime "created_at", null: false
    t.datetime "updated_at", null: false
    t.index ["name", "period", "period_start", "dims"], name: "idx_agentkit_metrics_unique", unique: true
  end

  create_table "agentkit_run_steps", force: :cascade do |t|
    t.bigint "run_id", null: false
    t.string "step_key", null: false
    t.string "step_name", null: false
    t.string "kind", null: false
    t.string "status", default: "pending", null: false
    t.integer "position"
    t.bigint "parent_step_id"
    t.integer "pending_count"
    t.jsonb "input", default: {}, null: false
    t.jsonb "output", default: {}, null: false
    t.jsonb "attempts", default: [], null: false
    t.integer "attempt_count", default: 0
    t.jsonb "usage", default: {}, null: false
    t.text "error"
    t.datetime "started_at"
    t.datetime "finished_at"
    t.datetime "timeout_at"
    t.datetime "created_at", null: false
    t.datetime "updated_at", null: false
    t.string "tenant_key", default: "__global__", null: false
    t.index ["parent_step_id"], name: "index_agentkit_run_steps_on_parent_step_id"
    t.index ["run_id", "status"], name: "index_agentkit_run_steps_on_run_id_and_status"
    t.index ["run_id", "step_key"], name: "index_agentkit_run_steps_on_run_id_and_step_key", unique: true
    t.index ["run_id"], name: "index_agentkit_run_steps_on_run_id"
    t.index ["tenant_key"], name: "index_agentkit_run_steps_on_tenant_key"
  end

  create_table "agentkit_runs", force: :cascade do |t|
    t.string "flow_name", null: false
    t.integer "flow_version", default: 1, null: false
    t.uuid "run_id", null: false
    t.string "status", default: "pending", null: false
    t.jsonb "input", default: {}, null: false
    t.jsonb "output", default: {}, null: false
    t.jsonb "context", default: {}, null: false
    t.string "idempotency_key"
    t.bigint "account_id"
    t.bigint "user_id"
    t.string "tenant_key", default: "__global__", null: false
    t.string "subject_type"
    t.bigint "subject_id"
    t.datetime "started_at"
    t.datetime "finished_at"
    t.datetime "deadline_at"
    t.jsonb "error"
    t.integer "steps_total", default: 0
    t.integer "steps_completed", default: 0
    t.decimal "cost_usd", precision: 12, scale: 6, default: "0.0"
    t.datetime "created_at", null: false
    t.datetime "updated_at", null: false
    t.index ["flow_name", "status"], name: "index_agentkit_runs_on_flow_name_and_status"
    t.index ["run_id"], name: "index_agentkit_runs_on_run_id", unique: true
    t.index ["subject_type", "subject_id"], name: "index_agentkit_runs_on_subject"
    t.index ["tenant_key", "idempotency_key"], name: "idx_agentkit_runs_tenant_idempotency", unique: true, where: "(idempotency_key IS NOT NULL)"
    t.index ["tenant_key"], name: "index_agentkit_runs_on_tenant_key"
  end

  create_table "agentkit_suggestions", force: :cascade do |t|
    t.string "suggestion_type", null: false
    t.string "title", null: false
    t.text "description"
    t.string "priority", default: "medium", null: false
    t.string "status", default: "pending", null: false
    t.string "source_agent"
    t.jsonb "payload", default: {}, null: false
    t.string "suggestable_type"
    t.bigint "suggestable_id"
    t.bigint "user_id"
    t.bigint "account_id"
    t.string "tenant_key", default: "__global__", null: false
    t.string "idempotency_key"
    t.string "prompt_id"
    t.integer "prompt_version"
    t.string "model"
    t.uuid "run_id"
    t.string "gate_key"
    t.jsonb "metadata", default: {}, null: false
    t.datetime "resolved_at"
    t.datetime "expires_at"
    t.datetime "created_at", null: false
    t.datetime "updated_at", null: false
    t.bigint "experiment_id"
    t.string "experiment_arm"
    t.string "operation_namespace"
    t.string "arguments_digest"
    t.integer "lock_version", default: 0, null: false
    t.string "execution_error_code"
    t.datetime "execution_started_at"
    t.datetime "execution_finished_at"
    t.index ["experiment_id"], name: "index_agentkit_suggestions_on_experiment_id"
    t.index ["gate_key"], name: "index_agentkit_suggestions_on_gate_key"
    t.index ["status", "priority"], name: "index_agentkit_suggestions_on_status_and_priority"
    t.index ["suggestable_type", "suggestable_id"], name: "index_agentkit_suggestions_on_suggestable"
    t.index ["tenant_key", "operation_namespace", "idempotency_key"], name: "idx_agentkit_suggestions_durable_idempotency", unique: true, where: "(idempotency_key IS NOT NULL)"
    t.index ["tenant_key", "status", "execution_started_at"], name: "idx_agentkit_suggestions_execution"
    t.index ["tenant_key", "status"], name: "index_agentkit_suggestions_on_tenant_key_and_status"
    t.index ["tenant_key"], name: "index_agentkit_suggestions_on_tenant_key"
  end

  create_table "agentkit_teams", force: :cascade do |t|
    t.string "tenant_key", default: "__global__", null: false
    t.string "name", null: false
    t.text "description"
    t.bigint "owner_id"
    t.bigint "account_id"
    t.jsonb "metadata", default: {}, null: false
    t.datetime "created_at", null: false
    t.datetime "updated_at", null: false
    t.index ["account_id"], name: "index_agentkit_teams_on_account_id"
    t.index ["tenant_key", "name"], name: "idx_agentkit_teams_tenant_name", unique: true
    t.index ["tenant_key"], name: "index_agentkit_teams_on_tenant_key"
  end

  create_table "agentkit_trace_phases", force: :cascade do |t|
    t.bigint "trace_id", null: false
    t.integer "position", null: false
    t.string "name", null: false
    t.jsonb "data", default: {}, null: false
    t.datetime "occurred_at", null: false
    t.datetime "created_at", null: false
    t.datetime "updated_at", null: false
    t.string "tenant_key", default: "__global__", null: false
    t.index ["tenant_key"], name: "index_agentkit_trace_phases_on_tenant_key"
    t.index ["trace_id", "position"], name: "index_agentkit_trace_phases_on_trace_id_and_position"
    t.index ["trace_id"], name: "index_agentkit_trace_phases_on_trace_id"
  end

  create_table "agentkit_traces", force: :cascade do |t|
    t.uuid "trace_id", null: false
    t.string "kind", null: false
    t.string "trigger", null: false
    t.string "status", default: "running", null: false
    t.uuid "run_id"
    t.string "tenant_key", default: "__global__", null: false
    t.jsonb "meta", default: {}, null: false
    t.datetime "started_at"
    t.datetime "finished_at"
    t.integer "duration_ms"
    t.datetime "created_at", null: false
    t.datetime "updated_at", null: false
    t.index ["kind", "started_at"], name: "index_agentkit_traces_on_kind_and_started_at"
    t.index ["run_id"], name: "index_agentkit_traces_on_run_id"
    t.index ["tenant_key"], name: "index_agentkit_traces_on_tenant_key"
    t.index ["trace_id"], name: "index_agentkit_traces_on_trace_id", unique: true
  end

  create_table "agentkit_usage_events", force: :cascade do |t|
    t.string "kind", null: false
    t.string "model"
    t.string "agent"
    t.string "tenant_key", default: "__global__", null: false
    t.uuid "run_id"
    t.integer "input_tokens", default: 0
    t.integer "output_tokens", default: 0
    t.decimal "cost_usd", precision: 12, scale: 6, default: "0.0"
    t.datetime "occurred_at", null: false
    t.datetime "created_at", null: false
    t.datetime "updated_at", null: false
    t.index ["kind", "occurred_at"], name: "index_agentkit_usage_events_on_kind_and_occurred_at"
    t.index ["tenant_key", "occurred_at"], name: "index_agentkit_usage_events_on_tenant_key_and_occurred_at"
  end

  create_table "agentkit_watchtower_issues", force: :cascade do |t|
    t.string "tenant_key", default: "__global__", null: false
    t.string "fingerprint", null: false
    t.string "detector", null: false
    t.string "severity", null: false
    t.string "status", default: "open", null: false
    t.string "subject_type"
    t.string "subject_id"
    t.jsonb "evidence", default: {}, null: false
    t.datetime "first_seen_at", null: false
    t.datetime "last_seen_at", null: false
    t.datetime "resolved_at"
    t.datetime "created_at", null: false
    t.datetime "updated_at", null: false
    t.index ["tenant_key", "fingerprint"], name: "idx_agentkit_watchtower_fingerprint", unique: true
    t.index ["tenant_key", "status", "severity"], name: "idx_agentkit_watchtower_status"
  end

  create_table "agentkit_wiki_pages", force: :cascade do |t|
    t.string "tenant_key", default: "__global__", null: false
    t.bigint "account_id"
    t.bigint "asset_id", null: false
    t.string "title", null: false
    t.text "content", null: false
    t.jsonb "links", default: [], null: false
    t.string "status", default: "ready", null: false
    t.datetime "created_at", null: false
    t.datetime "updated_at", null: false
    t.index ["account_id"], name: "index_agentkit_wiki_pages_on_account_id"
    t.index ["asset_id", "title"], name: "index_agentkit_wiki_pages_on_asset_id_and_title"
    t.index ["asset_id"], name: "index_agentkit_wiki_pages_on_asset_id"
    t.index ["tenant_key"], name: "index_agentkit_wiki_pages_on_tenant_key"
  end

  create_table "change_labels", force: :cascade do |t|
    t.bigint "change_id", null: false
    t.string "category", null: false
    t.string "value", null: false
    t.string "source", null: false
    t.float "confidence"
    t.string "reason"
    t.datetime "created_at", null: false
    t.datetime "updated_at", null: false
    t.index ["change_id", "category", "source"], name: "index_change_labels_on_change_id_and_category_and_source", unique: true
    t.index ["change_id"], name: "index_change_labels_on_change_id"
  end

  create_table "changes", force: :cascade do |t|
    t.string "repo", null: false
    t.string "ref", null: false
    t.string "pusher", null: false
    t.string "base_sha"
    t.string "head_sha", null: false
    t.string "state", default: "pending", null: false
    t.float "risk_score"
    t.jsonb "risk_features", default: {}, null: false
    t.jsonb "classification", default: {}, null: false
    t.jsonb "review_reasons", default: [], null: false
    t.bigint "gate_decision_id", null: false
    t.bigint "merge_batch_id"
    t.datetime "review_requested_at"
    t.datetime "reviewed_at"
    t.string "reviewer"
    t.string "outcome"
    t.datetime "created_at", null: false
    t.datetime "updated_at", null: false
    t.jsonb "report", default: {}, null: false
    t.index ["gate_decision_id"], name: "index_changes_on_gate_decision_id"
    t.index ["merge_batch_id"], name: "index_changes_on_merge_batch_id"
    t.index ["repo", "state", "created_at"], name: "index_changes_on_repo_and_state_and_created_at"
  end

  create_table "gate_decisions", force: :cascade do |t|
    t.string "repo", null: false
    t.string "ref", null: false
    t.string "pusher", null: false
    t.string "old_sha"
    t.string "new_sha"
    t.string "verdict", null: false
    t.string "review"
    t.jsonb "reasons", default: [], null: false
    t.jsonb "review_reasons", default: [], null: false
    t.integer "commit_count"
    t.jsonb "changed_paths", default: [], null: false
    t.datetime "created_at", null: false
    t.datetime "updated_at", null: false
    t.index ["repo", "created_at"], name: "index_gate_decisions_on_repo_and_created_at"
    t.index ["verdict"], name: "index_gate_decisions_on_verdict"
  end

  create_table "label_overrides", force: :cascade do |t|
    t.bigint "change_id", null: false
    t.string "category", null: false
    t.string "previous_value"
    t.string "new_value", null: false
    t.string "action", null: false
    t.string "reason"
    t.string "reviewer"
    t.datetime "created_at", null: false
    t.datetime "updated_at", null: false
    t.index ["change_id"], name: "index_label_overrides_on_change_id"
  end

  create_table "merge_batches", force: :cascade do |t|
    t.string "repo", null: false
    t.string "state", default: "running", null: false
    t.integer "size"
    t.integer "ci_runs", default: 0, null: false
    t.string "base_sha"
    t.string "merged_sha"
    t.jsonb "log", default: [], null: false
    t.datetime "created_at", null: false
    t.datetime "updated_at", null: false
  end

  add_foreign_key "agentkit_action_decisions", "agentkit_action_proposals", column: "proposal_id"
  add_foreign_key "agentkit_action_outboxes", "agentkit_action_proposals", column: "proposal_id"
  add_foreign_key "agentkit_action_outcomes", "agentkit_action_proposals", column: "proposal_id"
  add_foreign_key "agentkit_artifacts", "agentkit_runs", column: "run_id"
  add_foreign_key "agentkit_asset_bindings", "agentkit_memory_assets", column: "asset_id"
  add_foreign_key "agentkit_code_symbols", "agentkit_memory_assets", column: "asset_id"
  add_foreign_key "agentkit_decisions", "agentkit_experiments", column: "experiment_id"
  add_foreign_key "agentkit_decisions", "agentkit_suggestions", column: "suggestion_id"
  add_foreign_key "agentkit_execution_attempts", "agentkit_action_proposals", column: "proposal_id"
  add_foreign_key "agentkit_experiments", "agentkit_findings", column: "finding_id"
  add_foreign_key "agentkit_graph_edges", "agentkit_graph_snapshots", column: "snapshot_id"
  add_foreign_key "agentkit_graph_nodes", "agentkit_graph_snapshots", column: "snapshot_id"
  add_foreign_key "agentkit_graph_snapshots", "agentkit_memory_assets", column: "asset_id"
  add_foreign_key "agentkit_memory_assets", "agentkit_teams", column: "team_id"
  add_foreign_key "agentkit_run_steps", "agentkit_runs", column: "run_id"
  add_foreign_key "agentkit_suggestions", "agentkit_experiments", column: "experiment_id"
  add_foreign_key "agentkit_trace_phases", "agentkit_traces", column: "trace_id"
  add_foreign_key "agentkit_wiki_pages", "agentkit_memory_assets", column: "asset_id"
  add_foreign_key "change_labels", "changes"
  add_foreign_key "changes", "gate_decisions"
  add_foreign_key "label_overrides", "changes"
end
