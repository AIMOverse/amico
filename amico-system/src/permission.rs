//! Permission system for secure resource access.

/// Permission system for secure resource access.
///
/// Permissions gate access to system resources. Before a tool or side
/// effect touches a resource, the runtime checks whether the necessary
/// permission has been granted.
pub trait Permission<R> {
    /// Check if access to a resource is permitted
    fn check(&self, resource: &R) -> bool;

    /// Grant permission to access a resource
    fn grant(&mut self, resource: R);

    /// Revoke permission to access a resource
    fn revoke(&mut self, resource: &R);
}

/// Permission types for system resources
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ResourcePermission {
    FileRead(String),
    FileWrite(String),
    NetworkAccess(String),
    ProcessExecution,
}

/// Simple permission checker
#[derive(Debug, Clone, Default)]
pub struct PermissionChecker {
    granted: Vec<ResourcePermission>,
}

impl PermissionChecker {
    pub fn new() -> Self {
        Self {
            granted: Vec::new(),
        }
    }

    pub fn grant_all(&mut self) {
        // Grant all permissions (use with caution!)
        self.granted.push(ResourcePermission::ProcessExecution);
    }
}

impl Permission<ResourcePermission> for PermissionChecker {
    fn check(&self, resource: &ResourcePermission) -> bool {
        self.granted.contains(resource)
    }

    fn grant(&mut self, resource: ResourcePermission) {
        if !self.granted.contains(&resource) {
            self.granted.push(resource);
        }
    }

    fn revoke(&mut self, resource: &ResourcePermission) {
        self.granted.retain(|r| r != resource);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permission_grant_and_check() {
        let mut checker = PermissionChecker::new();
        let perm = ResourcePermission::FileRead("/tmp".to_string());

        assert!(!checker.check(&perm));
        checker.grant(perm.clone());
        assert!(checker.check(&perm));
    }

    #[test]
    fn test_permission_revoke() {
        let mut checker = PermissionChecker::new();
        let perm = ResourcePermission::NetworkAccess("https://example.com".to_string());

        checker.grant(perm.clone());
        assert!(checker.check(&perm));

        checker.revoke(&perm);
        assert!(!checker.check(&perm));
    }

    #[test]
    fn test_permission_no_duplicates() {
        let mut checker = PermissionChecker::new();
        let perm = ResourcePermission::ProcessExecution;

        checker.grant(perm.clone());
        checker.grant(perm.clone());
        // Revoke once should remove all instances
        checker.revoke(&perm);
        assert!(!checker.check(&perm));
    }
}
