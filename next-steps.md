# Next Steps

- [x] Refactor service impls to hold reference to `State<_>` instead of `Arc`
  - Each service uses a Fairing?
    ```rust
    Rocket::build()
        .manage(Db::fairing())
        .attach(AdHoc::OnLaunch(|rocket: &Rocket<Build>| {
            auth_service_factory(&rocket)?
        }))
        .attach(AdHoc::OnLaunch(|rocket: &Rocket<Build>| {
            member_service_factory(&rocket)?
        }))
        .launch()
        .await?
    ```
- [ ] Use Figment for config
