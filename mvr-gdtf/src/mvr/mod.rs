use std::ops;

mod schema;

pub use schema::*;

impl ops::Deref for Layers {
    type Target = Vec<Layer>;

    fn deref(&self) -> &Self::Target {
        &self.layer
    }
}

impl ops::Deref for Addresses {
    type Target = Vec<crate::mvr::Address>;

    fn deref(&self) -> &Self::Target {
        &self.address
    }
}

impl ops::Deref for Alignments {
    type Target = Vec<crate::mvr::Alignment>;

    fn deref(&self) -> &Self::Target {
        &self.alignment
    }
}

impl ops::Deref for Connections {
    type Target = Vec<crate::mvr::Connection>;

    fn deref(&self) -> &Self::Target {
        &self.connection
    }
}

impl ops::Deref for CustomCommands {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.custom_command
    }
}

impl ops::Deref for Geometries {
    type Target = Vec<crate::mvr::Geometry3D>;

    fn deref(&self) -> &Self::Target {
        &self.geometry_3d
    }
}

impl ops::Deref for Mappings {
    type Target = Vec<crate::mvr::Mapping>;

    fn deref(&self) -> &Self::Target {
        &self.mapping
    }
}

impl ops::Deref for Overwrites {
    type Target = Vec<crate::mvr::Overwrite>;

    fn deref(&self) -> &Self::Target {
        &self.overwrite
    }
}

impl ops::Deref for Projections {
    type Target = Vec<crate::mvr::Projection>;

    fn deref(&self) -> &Self::Target {
        &self.projection
    }
}

impl ops::Deref for Protocols {
    type Target = Vec<crate::mvr::Protocol>;

    fn deref(&self) -> &Self::Target {
        &self.protocol
    }
}

impl ops::Deref for Sources {
    type Target = Vec<crate::mvr::Source>;

    fn deref(&self) -> &Self::Target {
        &self.source
    }
}

impl ops::Deref for UserData {
    type Target = Vec<crate::mvr::Data>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl ops::Deref for AuxData {
    type Target = Vec<crate::mvr::BasicChildListAttribute>;

    fn deref(&self) -> &Self::Target {
        &self.class
    }
}
