Rails.application.routes.draw do
  get "dashboard", to: "dashboard#show"
  get "frameline", to: "product#show", as: :product
  resource :reviewer_session, only: %i[new create destroy], path: "frameline/reviewer"
  post "frameline/reviews/:review_id/:decision", to: "decisions#create", as: :decide_review,
                                                 constraints: { decision: /approve|reject/ }
  get "frameline/golden/:rev/:name", to: "product#golden", as: :product_golden,
                                     constraints: { rev: /[0-9a-f]{40}/, name: /[A-Za-z0-9_-][A-Za-z0-9_.-]*/ }, format: false
  get "frameline/renders/:sha/:name", to: "product#rendered", as: :product_render,
                                      constraints: { sha: /[0-9a-f]{40}/, name: /[A-Za-z0-9_-][A-Za-z0-9_.-]*\.(png|mp4)/ }, format: false
  root "dashboard#show"

  resources :reviews, only: :index do
    member do
      post :approve
      post :reject
      post :label
    end
  end

  namespace :gate do
    post "push", to: "pushes#create"
  end
  mount Agentkit::Engine => "/agentkit"
  # Define your application routes per the DSL in https://guides.rubyonrails.org/routing.html

  # Reveal health status on /up that returns 200 if the app boots with no exceptions, otherwise 500.
  # Can be used by load balancers and uptime monitors to verify that the app is live.
  get "up" => "rails/health#show", as: :rails_health_check

  # Render dynamic PWA files from app/views/pwa/* (remember to link manifest in application.html.erb)
  # get "manifest" => "rails/pwa#manifest", as: :pwa_manifest
  # get "service-worker" => "rails/pwa#service_worker", as: :pwa_service_worker

  # Defines the root path route ("/")
  # root "posts#index"
end
