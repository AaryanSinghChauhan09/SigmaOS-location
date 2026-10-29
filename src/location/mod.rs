#![allow(dead_code)]
// SigmaOS Location Module
// Location services
// Zero-dependency implementation - no external libraries required

use core::fmt;
use std::boxed::Box;
use std::string::{String, ToString};
use std::vec::Vec;

/// Error type for the Location module
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LocationError {
    /// Operation not supported
    NotSupported,
    /// Invalid parameter
    InvalidParam,
    /// Resource not found
    NotFound,
    /// Permission denied
    PermissionDenied,
    /// Out of memory
    OutOfMemory,
    /// I/O error
    IoError,
    /// Unknown error
    Unknown,
}

impl fmt::Display for LocationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotSupported => write!(f, "Location: operation not supported"),
            Self::InvalidParam => write!(f, "Location: invalid parameter"),
            Self::NotFound => write!(f, "Location: resource not found"),
            Self::PermissionDenied => write!(f, "Location: permission denied"),
            Self::OutOfMemory => write!(f, "Location: out of memory"),
            Self::IoError => write!(f, "Location: I/O error"),
            Self::Unknown => write!(f, "Location: unknown error"),
        }
    }
}

/// Result type alias for Location operations
pub type LocationResult<T> = Result<T, LocationError>;

/// LocationManager - primary abstraction for this module
#[derive(Debug, Clone)]
pub struct LocationManager {
    pub id: u64,
    pub name: String,
    pub enabled: bool,
}

impl LocationManager {
    /// Create a new LocationManager with the given name
    pub fn new(name: &str) -> Self {
        Self {
            id: 0,
            name: name.into(),
            enabled: false,
        }
    }

    /// Enable this resource
    pub fn enable(&mut self) -> LocationResult<()> {
        self.enabled = true;
        Ok(())
    }

    /// Disable this resource
    pub fn disable(&mut self) -> LocationResult<()> {
        self.enabled = false;
        Ok(())
    }

    /// Check if enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

/// Manager for Location resources
#[derive(Debug)]
pub struct GpsData {
    resources: Vec<LocationManager>,
    initialized: bool,
}

impl GpsData {
    /// Create a new GpsData
    pub fn new() -> Self {
        Self {
            resources: Vec::new(),
            initialized: false,
        }
    }

    /// Initialize the Location subsystem
    pub fn init(&mut self) -> LocationResult<()> {
        self.initialized = true;
        Ok(())
    }

    /// Add a resource
    pub fn add(&mut self, resource: LocationManager) -> LocationResult<u64> {
        if !self.initialized {
            return Err(LocationError::NotSupported);
        }
        let id = self.resources.len() as u64;
        self.resources.push(resource);
        Ok(id)
    }

    /// Get resource by ID
    pub fn get(&self, id: u64) -> Option<&LocationManager> {
        self.resources.get(id as usize)
    }

    /// Get mutable resource by ID
    pub fn get_mut(&mut self, id: u64) -> Option<&mut LocationManager> {
        self.resources.get_mut(id as usize)
    }

    /// List all resources
    pub fn list(&self) -> &[LocationManager] {
        &self.resources
    }

    /// Check if initialized
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    /// Shutdown the subsystem
    pub fn shutdown(&mut self) -> LocationResult<()> {
        self.initialized = false;
        self.resources.clear();
        Ok(())
    }
}

impl Default for GpsData {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_location_manager_lifecycle() {
        // Arrange
        let mut manager = LocationManager::new("gps_core");
        assert_eq!(manager.name, "gps_core");
        assert!(!manager.is_enabled());

        // Act
        let enable_res = manager.enable();
        // Assert
        assert!(enable_res.is_ok());
        assert!(manager.is_enabled());

        // Act
        let disable_res = manager.disable();
        // Assert
        assert!(disable_res.is_ok());
        assert!(!manager.is_enabled());
    }

    #[test]
    fn test_location_manager_init() {
        // Arrange
        let mut manager = GpsData::new();
        assert!(!manager.is_initialized());

        // Act
        let init_res = manager.init();

        // Assert
        assert!(init_res.is_ok());
        assert!(manager.is_initialized());
    }

    #[test]
    fn test_location_resource_add_and_query() {
        // Arrange
        let mut manager = GpsData::new();
        manager.init().unwrap();
        let resource = LocationManager::new("geoloc_sensor");

        // Act
        let id = manager.add(resource).unwrap();

        // Assert
        assert_eq!(id, 0);
        let item = manager.get(0);
        assert!(item.is_some());
        assert_eq!(item.unwrap().name, "geoloc_sensor");
        assert_eq!(manager.list().len(), 1);
    }

    #[test]
    fn test_location_resource_add_uninitialized_fails() {
        // Arrange: uninitialized manager
        let mut manager = GpsData::new();
        let resource = LocationManager::new("sensor_fail");

        // Act
        let res = manager.add(resource);

        // Assert: Defensive programming - should reject when uninitialized
        assert_eq!(res, Err(LocationError::NotSupported));
    }

    #[test]
    fn test_location_manager_shutdown() {
        // Arrange
        let mut manager = GpsData::new();
        manager.init().unwrap();
        manager.add(LocationManager::new("res1")).unwrap();
        assert_eq!(manager.list().len(), 1);

        // Act
        let shut_res = manager.shutdown();

        // Assert
        assert!(shut_res.is_ok());
        assert!(!manager.is_initialized());
        assert_eq!(manager.list().len(), 0);
    }

    #[test]
    fn test_location_error_formatting() {
        // Arrange & Act & Assert
        assert_eq!(format!("{}", LocationError::NotSupported), "Location: operation not supported");
        assert_eq!(format!("{}", LocationError::NotFound), "Location: resource not found");
        assert_eq!(format!("{}", LocationError::PermissionDenied), "Location: permission denied");
    }
}
