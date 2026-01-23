## Routing

Routing in Makara is really easy. There are only 3 steps.

1. Specifying route name on `root` widget.
2. Register route.
3. Navigating.

### Specifying route
Just like I mentioned in previous chapter about [hierarchy](gettingstart.html#understand-makara-hierarchy), 
we need to use `root` widget as a starting point for UI hierarchy.

If you want to have multiple pages or views, you need to create multiple starting points. In other words, 
each `root` defines one page or view.

For example, let's say you want to have **Home** view and **About Us** view.

```rust
fn main() {
    App::new()
        .add_plugins(MakaraPlugin::default())
        .add_systems(Startup, (setup_home_view, setup_about_us_view))
        .run();
}

fn setup_home_view(mut commands: Commands) {
    commands.spawn(
        root_!(route: "home"; [ text_!("This is home view") ])
    );
}

fn setup_about_us_view(mut commands: Commands) {
    commands.spawn(
        root_!(route: "about-us"; [ text_!("This is about us view") ])
    );
}
```
We can specify route name for each view by using `route` property.

### Registering Routes
After we defined route on each view, we need to register those routes.
We can do so by using `Router` resource.

```rust
fn main() {
    App::new()
        .add_plugins(MakaraPlugin::default())
        .add_systems(Startup, (
            setup_home_view, 
            setup_about_us_view, 
            setup_routes // add here
        ))
        .run();
}

fn setup_routes(mut router: ResMut<Router>) {
    router.register_routes(["home", "about-us"]);
    
    // define which route you want to see when the application started
    router.default_route("home", ());
}

// setup view systems..
```
You may also want to set the default route. If you don't, the first route in the list
will be set to default automatically.

### Route Navigation
Let's update the above examples to demonstrate how to nagivate to specific route.

```rust
fn setup_home_view(mut commands: Commands) {
    commands.spawn(
        root_!(route: "home"; [
            text_!("This is home view"),
            button_!(
                "Go to about us view";
                on: |_clicked: On<Clicked>, mut router: ResMut<Router>| {
                    router.nagivate("about-us", ());
                }
            )
        ])
    );
}

fn setup_about_us_view(mut commands: Commands) {
    commands.spawn(
        root_!(route: "about-us"; [
            text_!("This is about us view"),
            button_!(
                "Go to home view";
                on: |_clicked: On<Clicked>, mut router: ResMut<Router>| {
                    router.nagivate("home", ());
                }
            )
        ])
    );
}
```
We can navigate to any route by calling `navigate` method.

### Route Params
Of course, each route can have one or multiple params.

For example, let's say we have a view called **product-detail**, but this view is dynamic based
on product id or name.

```rust
fn setup_home_view(mut commands: Commands) {
    commands.spawn(
        root_!(route: "home"; [
            text_!("This is home view"),
            button_!(
                "See product";
                on: |_clicked: On<Clicked>, mut router: ResMut<Router>| {
                    router.nagivate(
                        "product-detail", 
                        Param::new()
                            .value("id", "1234") // just random product id
                            .value("name", "Iphone 17 256GB") // product name
                    );
                }
            )
        ])
    );
}

fn setup_product_detail_view(mut commands: Commands) { 
    commands.spawn(
        root_!(
            route: "product-detail";
            
            on: |page_loaded: On<PageLoaded>, mut text_q: TextQuery| {
                if let Some(text) = text_q.find_by_id("product-name") {
                    if let Some(name) = page_loaded.param.get("name") { 
                        text.text.value.0 = name.clone();
                    } 
                }
            };
            
            [ text_!("Product name: Unknown", id: "product-name") ]
        )
    );
}
```

See [Router](https://docs.rs/makara/latest/makara/routers/struct.Router.html) and 
[Param](https://docs.rs/makara/latest/makara/routers/struct.Param.html) for more info.
