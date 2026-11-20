#[cxx::bridge]
pub mod ffi {
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Vec3 {
        pub x: f32,
        pub y: f32,
        pub z: f32,
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Transform {
        pub translation: Vec3,
        pub rotation: [Vec3; 3], // row-major 3x3
        pub scale: f32,
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct BoundingSphere {
        pub center: Vec3,
        pub radius: f32,
    }

    /// One bounding volume, in pre-order: a union's `children` records follow it.
    ///
    /// `kind` is `BoundVolumeType`. `values` by kind: sphere (center, radius), box (center,
    /// axis 1 to 3, extent 1 to 3), capsule (center, origin, extent, radius), half space
    /// (normal, constant, center). Unused values are 0.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct BoundingVolumeFfi {
        pub kind: u32,
        pub values: [f32; 15],
        pub children: u32,
    }

    #[namespace = "nifly"]
    extern "C++" {
        include!("Object3d.hpp");
    }
    // Lets other bridges pass `Vec<Vec3>`.
    impl Vec<Vec3> {}
    impl Vec<BoundingVolumeFfi> {}
    impl Vec<BoundingSphere> {}
}
