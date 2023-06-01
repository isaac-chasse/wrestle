// ! This is where the evaluation of all the different compatibilities will live
// ! We additionally NEED error handling for this
use crate::VersionSpecification;

// matching for operations
fn matches_exact(tgt_ver: VersionSpecification, src_ver: VersionSpecification) -> bool {
    // all versions major, minor, patch must match exactly
    if src_ver.major != tgt_ver.major {
        return false;
    }

    if let Some(minor) = tgt_ver.minor {
        if src_ver.minor != minor {
            return false;
        }
    }

    if let Some(patch) = tgt_ver.patch {
        if src_ver.patch != patch {
            return false;
        }
    }

    // src_ver.pre == tgt_ver.pre
    src_ver == tgt_ver
}

fn matches_greater(tgt_ver: VersionSpecification, src: VersionSpecification) -> bool {
    // either major, minor, and/or patch must be greater
    if src_ver.major != tgt_ver.major {
        return src_ver.major > tgt_ver.major;
    }

    match tgt_ver.minor {
        None => return false,
        Some(minor) => {
            if src_ver.minor != minor {
                return src_ver > minor;
            }
        }
    }

    match tgt_ver.patch {
        None => return false,
        Some(patch) => {
            if src_ver.patch != patch {
                return src_ver > patch;
            }
        }
    }

    src_ver > tgt_ver
}

fn matches_less(tgt_ver: VersionSpecification, src: VersionSpecification) -> bool {
    // either major, minor, and/or patch must be less
    if src_ver.major != tgt_ver.major {
        return src_ver.major < tgt_ver.major;
    }

    match tgt_ver.minor {
        None => return false,
        Some(minor) => {
            if src_ver.minor != minor {
                return src_ver < minor;
            }
        }
    }

    match tgt_ver.patch {
        None => return false,
        Some(patch) => {
            if src_ver.patch != patch {
                return src_ver < patch;
            }
        }
    }

    src_ver < tgt_ver
}

fn matches_tilde(tgt_ver: VersionSpecification, src: VersionSpecification) -> bool {
    // any version greater than or equal to major, minor, patch but less than
    // the next major release
    if src_ver.major != tgt_ver.major {
        return false;
    }

    if let Some(minor) = tgt_ver.minor {
        if src_ver.minor != minor {
            return false;
        }
    }

    match tgt_ver.patch {
        None => return false,
        Some(patch) => {
            if src_ver.patch != patch {
                return src_ver > patch;
            }
        }
    }

    src_ver >= tgt_ver
}

fn matches_caret(tgt_ver: VersionSpecification, src: VersionSpecification) -> bool {
    if src_ver.major != tgt_ver.major {
        return false;
    }

    let minor = match tgt_ver.minor {
        None => return true,
        Some(minor) => minor,
    };

    let patch = match tgt_ver.patch {
        None => {
            if tgt_ver.major > 0 {
                return src_ver.minor >= minor;
            } else {
                return src_ver.minor == minor;
            }
        }
        Some(patch) => patch,
    };

    if tgt_ver.major > 0 {
        if src_ver.minor != minor {
            return src_ver.minor > minor;
        } else if src_ver.patch != patch {
            return src_ver.patch > patch;
        }
    } else if minor > 0 {
        if src_ver.minor != minor {
            return false;
        } else if src_ver.patch != patch {
            return src_ver.patch > patch;
        }
    } else if src_ver.minor != minor || src_ver.patch != patch {
        return false;
    }

    src_ver >= tgt_ver
}