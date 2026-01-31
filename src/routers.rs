use bevy::prelude::*;

use super::*;

use std::collections::HashMap;

/// Router param.
#[derive(Debug, Default, Clone)]
pub struct Param {
    pub maps: HashMap<String, String>
}

impl Param {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn none() -> Self {
        Self::default()
    }

    pub fn value(mut self, key: &str, value: &str) -> Self {
        self.maps.insert(key.to_string(), value.to_string());
        self
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.maps.get(key).map(|s| s.as_str())
    }
}

impl From<()> for Param {
    fn from(_: ()) -> Self {
        Param::none()
    }
}

/// Rosource used to manage routing.
#[derive(Resource, Debug, Default)]
pub struct Router {
    pub route_names: HashMap<String, Param>,
    pub(crate) current_route: Option<(String, Param)>
}

impl Router {
    /// Register routes
    pub fn register_routes<I, S>(&mut self, routes: I)
    where
        I: IntoIterator<Item = S>,
        S: Into<String>
    {
        for (i, route) in routes.into_iter().enumerate() {
            let route_key: String = route.into();

            if i == 0 && self.current_route.is_none() {
                self.current_route = Some((route_key.clone(), Param::default()));
            }

            self.route_names.insert(route_key, Param::default());
        }
    }

    /// Set default route with provided param.
    /// If this method is not called, the first route will be used.
    pub fn default_route(&mut self, name: &str, param: impl Into<Param>) {
        if self.route_names.contains_key(name) {
            self.current_route = Some((name.to_string(), param.into()));
        }
    }

    /// Navigate to provided route with/without param.
    pub fn navigate(&mut self, name: &str, param: impl Into<Param>) {
        self.default_route(name, param.into());
    }

    /// Get current route and its param.
    pub fn get_current_route(&self) -> Option<(String, Param)> {
        self.current_route.clone()
    }
}

pub(crate) fn handle_match_route_to_root(
    router: Res<Router>,
    mut commands: Commands,
    mut roots: Query<(Entity, &mut Node, &RouteName), IsRootOnly>
) {
    if !router.is_changed() {
        return;
    }

    for (entity, mut node, page) in roots.iter_mut() {
        if let Some(current_page) = &router.current_route {
            if page.0 == current_page.0 {
                node.display = Display::default();

                commands.trigger(PageLoaded {
                    entity,
                    name: page.0.clone(),
                    param: current_page.1.clone()
                });

                commands.trigger(RouteChanged {
                    route: current_page.0.clone(),
                    param: current_page.1.clone()
                });
            }
            else {
                node.display = Display::None;
            }
        }
    }
}
