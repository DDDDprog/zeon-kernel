/*

pub struct Capabilities {
    effective: CapabilitiesFlags,
    permitted: CapabilitiesFlags,
    inheritable: CapabilitiesFlags,
    ambient: CapabilitiesFlags,
    bounding: CapabilitiesFlags,
}

impl Capabilities {
    pub fn new(
        effective: CapabilitiesFlags,
        permitted: CapabilitiesFlags,
        inheritable: CapabilitiesFlags,
        ambient: CapabilitiesFlags,
        bounding: CapabilitiesFlags,
    ) -> Self {
        Self {
            effective,
            permitted,
            inheritable,
            ambient,
            bounding,
        }
    }

    pub fn new_root() -> Self {
        Self {
            effective: CapabilitiesFlags::all(),
            permitted: CapabilitiesFlags::all(),
            inheritable: CapabilitiesFlags::all(),
            ambient: CapabilitiesFlags::all(),
            bounding: CapabilitiesFlags::all(),
        }
    }

    pub fn new_empty() -> Self {
        Self {
            effective: CapabilitiesFlags::empty(),
            permitted: CapabilitiesFlags::empty(),
            inheritable: CapabilitiesFlags::empty(),
            ambient: CapabilitiesFlags::empty(),
            bounding: CapabilitiesFlags::empty(),
        }
    }

    /// Convenience method for creating capabilities with a single capability
    pub fn new_cap(cap: CapabilitiesFlags) -> Self {
        Self {
            effective: cap,
            permitted: cap,
            inheritable: cap,
            ambient: cap,
            bounding: cap,
        }
    }

    /// Set the publicly mutable fields on capabilities
    pub fn set_public(
        &mut self,
        caller_caps: Capabilities,
        effective: CapabilitiesFlags,
        permitted: CapabilitiesFlags,
        inheritable: CapabilitiesFlags,
    ) -> Result<()> {
        // permitted should be a subset of self.permitted, and effective should be a subset of permitted
        // inheritable should be a subset of self.bounding, or caller's effective should contain CAP_SETPCAP
        if !self.permitted.contains(permitted)
            || !permitted.contains(effective)
            || (!self.bounding.contains(inheritable)
                && !caller_caps
                    .effective
                    .contains(CapabilitiesFlags::CAP_SETPCAP))
        {
            return Err(KernelError::NotPermitted);
        }
        self.effective = effective;
        self.permitted = permitted;
        self.inheritable = inheritable;
        Ok(())
    }

    pub fn effective(&self) -> CapabilitiesFlags {
        self.effective
    }

    pub fn permitted(&self) -> CapabilitiesFlags {
        self.permitted
    }

    pub fn inheritable(&self) -> CapabilitiesFlags {
        self.inheritable
    }

    pub fn ambient(&self) -> CapabilitiesFlags {
        self.ambient
    }

    pub fn ambient_mut(&mut self) -> &mut CapabilitiesFlags {
        &mut self.ambient
    }

    pub fn bounding(&self) -> CapabilitiesFlags {
        self.bounding
    }

    pub fn bounding_mut(&mut self) -> &mut CapabilitiesFlags {
        &mut self.bounding
    }

    /// Checks if a capability is effective, as in if it can be used.
    pub fn is_capable(&self, cap: CapabilitiesFlags) -> bool {
        self.effective.contains(cap)
    }

    /// Shortcut for returning EPERM if a capability is not effective.
    pub fn check_capable(&self, cap: CapabilitiesFlags) -> Result<()> {
        if !self.effective.contains(cap) {
            Err(KernelError::NotPermitted)
        } else {
            Ok(())
        }
    }
}
