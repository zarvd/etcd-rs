macro_rules! pbwrap_request {
    ($(#[$attr:meta])* $intern:ident => $name:ident) => {
        $(#[$attr])*
        pub struct $name {
            proto: crate::proto::etcdserverpb::$intern,
        }
        impl From<$name> for crate::proto::etcdserverpb::$intern {
            fn from(x: $name) -> Self {
                x.proto
            }
        }
    };
    ($(#[$attr:meta])* $name:ident) => {
        pbwrap_request!($(#[$attr])* $name => $name);
    }
}

macro_rules! pbwrap_response {
    ($(#[$attr:meta])* $intern:ident => $name:ident) => {
        $(#[$attr])*
        #[derive(Debug)]
        pub struct $name {
            proto: crate::proto::etcdserverpb::$intern,
        }
        impl From<crate::proto::etcdserverpb::$intern> for $name {
            fn from(resp: crate::proto::etcdserverpb::$intern) -> Self {
                Self { proto: resp }
            }
        }
    };
    ($(#[$attr:meta])* $name:ident) => {
        pbwrap_response!($(#[$attr])* $name => $name);
    }
}
