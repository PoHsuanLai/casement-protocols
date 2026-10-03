//! Window vocabulary: `quire_window_v1`'s enums and the window enums of
//! `quire_desktop_window_v1`.

use super::code::wire_enum;

wire_enum! {
    /// What kind of window this is (`quire_window_v1.window_role`).
    WindowRole {
        /// An ordinary document or app window.
        Document = 0,
        /// A floating panel that serves a document window.
        Utility = 1,
        /// A dialog or sheet.
        Dialog = 2,
        /// Picture in picture.
        Pip = 3,
        /// A surface of the desktop shell.
        Shell = 4,
    }
}

wire_enum! {
    /// How a window blocks others (`quire_desktop_window_v1.modality`).
    Modality {
        /// Blocks nothing.
        None = 0,
        /// A sheet: blocks its parent window.
        WindowModal = 1,
        /// A dialog that blocks nothing.
        Modeless = 2,
    }
}

wire_enum! {
    /// A window's state without its data (`quire_desktop_window_v1.window_state_kind`).
    WindowStateKind {
        /// Placed by hand.
        Floating = 0,
        /// In a tile zone.
        Tiled = 1,
        /// Filling the usable area.
        Filled = 2,
        /// Full screen.
        FullScreen = 3,
        /// Minimised into the dock.
        Minimised = 4,
    }
}

wire_enum! {
    /// The region a tiled window fills (`quire_window_v1.tile_zone`).
    TileZone {
        /// The left half.
        Left = 0,
        /// The right half.
        Right = 1,
        /// The top half.
        Top = 2,
        /// The bottom half.
        Bottom = 3,
        /// The top-left quarter.
        TopLeft = 4,
        /// The top-right quarter.
        TopRight = 5,
        /// The bottom-left quarter.
        BottomLeft = 6,
        /// The bottom-right quarter.
        BottomRight = 7,
    }
}

wire_enum! {
    /// Where a window is asked to go (`quire_window_v1.place_target`): a tile zone (same codes
    /// as [`TileZone`]), or fill, centre or restore.
    PlaceTarget {
        /// The left half.
        Left = 0,
        /// The right half.
        Right = 1,
        /// The top half.
        Top = 2,
        /// The bottom half.
        Bottom = 3,
        /// The top-left quarter.
        TopLeft = 4,
        /// The top-right quarter.
        TopRight = 5,
        /// The bottom-left quarter.
        BottomLeft = 6,
        /// The bottom-right quarter.
        BottomRight = 7,
        /// The whole usable area, still a window.
        Fill = 8,
        /// Centred on its output at its own size.
        Centre = 9,
        /// The size and place before the last placement.
        Restore = 10,
    }
}

wire_enum! {
    /// A window's current placement, as its zoom control shows it
    /// (`quire_window_v1.placement_mark`).
    PlacementMark {
        /// Placed by hand, not tiled.
        Free = 0,
        /// The left half.
        Left = 1,
        /// The right half.
        Right = 2,
        /// The top half.
        Top = 3,
        /// The bottom half.
        Bottom = 4,
        /// The top-left quarter.
        TopLeft = 5,
        /// The top-right quarter.
        TopRight = 6,
        /// The bottom-left quarter.
        BottomLeft = 7,
        /// The bottom-right quarter.
        BottomRight = 8,
        /// The whole usable area.
        Fill = 9,
        /// Full screen in its own Space.
        FullScreen = 10,
    }
}

wire_enum! {
    /// How much a window stands out (`quire_window_v1.emphasis`).
    Emphasis {
        /// Takes keyboard input.
        Key = 0,
        /// The active app's main window while another of its windows is key.
        Main = 1,
        /// Neither key nor main.
        Inactive = 2,
    }
}

wire_enum! {
    /// Whether a window's client answers pings (`quire_desktop_window_v1.responsiveness`).
    Responsiveness {
        /// Answering pings.
        Responding = 0,
        /// Not answering pings.
        NotResponding = 1,
    }
}

wire_enum! {
    /// Who draws a window's shadow and corner clip (`quire_window_v1.frame_mode`).
    FrameMode {
        /// The client draws its own.
        Client = 0,
        /// The compositor draws the shadow and clips the corners.
        CompositorShadow = 1,
    }
}

wire_enum! {
    /// Whether a window's document has unsaved changes (`quire_window_v1.edited`).
    Edited {
        /// No unsaved changes.
        Saved = 0,
        /// Unsaved changes.
        Unsaved = 1,
    }
}

wire_enum! {
    /// Turn something on or off (`quire_desktop_window_v1.toggle`).
    Toggle {
        /// Off.
        Off = 0,
        /// On.
        On = 1,
    }
}

wire_enum! {
    /// Which Spaces a window is assigned to (`quire_desktop_window_v1.assign`).
    Assign {
        /// Every Space.
        AllSpaces = 0,
        /// The Space it is on.
        ThisSpace = 1,
        /// No assignment: it stays where it opens.
        None = 2,
    }
}

wire_enum! {
    /// Which Spaces show a window (`quire_desktop_window_v1.place_kind`).
    PlaceKind {
        /// One Space.
        OnSpace = 0,
        /// Every Space.
        AllSpaces = 1,
    }
}
