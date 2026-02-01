# COSMIC Connect Applet - UI/UX Enhancement Implementation Plan

## Issue Reference
GitHub Issue #119: Comprehensive UI/UX Review and Enhancement

## Executive Summary
This document outlines the implementation plan for UI/UX enhancements to the COSMIC Connect applet. The enhancements are prioritized based on user impact and implementation complexity.

## Current State Analysis

### Infrastructure Already in Place ✅
1. **Keyboard Navigation Framework**
   - FocusTarget enum with Search, Device, DeviceAction, MprisControl, Refresh
   - Tab/Shift+Tab navigation implemented (lines 6065-6066)
   - Arrow key navigation (lines 6067-6070)
   - Enter/Space activation (line 6071)
   - Ctrl+R (refresh), Ctrl+F (search focus) shortcuts
   - focus_next/previous/up/down/left/right methods (lines 6130-6234)

2. **Drag-and-Drop Foundation**
   - State variables: drag_hover_device, dragging_files (lines 421-422)
   - Message handlers: FileDragEnter, FileDragLeave, FileDropped, SetDragHoverDevice
   - Event subscription (lines 2800-2808)
   - Drop zone indicator logic (line 4497)
   - Single device file drop working (lines 2185-2211)

3. **Context Menu Infrastructure**
   - State tracking: context_menu_device, context_menu_transfer, context_menu_mpris
   - Message handlers: ShowContextMenu, CloseContextMenu (lines 2217-2223)
   - Context menu button in device card (lines 4786-4809)
   - Partial view method: device_context_menu_view (line 4487)

4. **Screen Share State Management**
   - ActiveScreenShare struct with quality, fps, audio, viewer_count (lines 456-466)
   - ScreenShareStarted/Stopped handlers (lines 2350-2375)
   - Pause/Resume support (lines 2387-2405)
   - Quality/FPS/Audio state updates (lines 2406-2450)
   - Screen share overlay view (line 3374)
   - Stats polling every 2 seconds (lines 6330-6362)

5. **File Transfer Tracking**
   - TransferState struct with progress, speed tracking (lines 314-324)
   - ReceivedFile history struct (lines 327-335)
   - active_transfers HashMap (line 382)
   - received_files_history Vec with MAX_RECEIVED_FILES_HISTORY=50 (lines 338-340)
   - Transfer progress indicators

### Gaps to Address 🔧

#### High Priority
1. **Keyboard Navigation Enhancement**
   - ❌ Tab order not optimized for all interactive elements
   - ❌ Focus indicators not visually prominent
   - ❌ Missing shortcuts documentation in UI
   - ❌ Context menu keyboard access incomplete

2. **File Sharing UX**
   - ❌ Drag-drop visual feedback minimal
   - ❌ No received files history view (data tracked but not displayed)
   - ❌ Transfer pause/cancel UI missing
   - ❌ File type icons not shown in transfer list
   - ❌ Batch file selection not implemented

3. **Screen Share Controls**
   - ⚠️ Overlay exists but lacks quality/fps controls in UI
   - ❌ Audio toggle backend not implemented (line 2444 TODO)
   - ❌ Switch source option missing
   - ❌ Quality settings require restart notification

4. **Context Menus**
   - ⚠️ Device context menu partially implemented
   - ❌ Transfer context menu not implemented
   - ❌ MPRIS context menu not implemented
   - ❌ Right-click activation missing

#### Medium Priority
5. **Accessibility**
   - ❌ ARIA labels/semantic markup
   - ❌ High contrast mode testing
   - ❌ Screen reader compatibility
   - ❌ Touch target sizes (44px minimum)

6. **Workflow Improvements**
   - ❌ Device favorites/pinning UI (config exists, no UI)
   - ❌ Notification preferences per device
   - ❌ First-run onboarding flow

## Implementation Phases

### Phase 1: Keyboard Navigation Polish (High Impact, Low Effort)
**Estimated Effort:** 1 day

#### Tasks
1. **Enhanced Focus Indicators**
   - Add visible focus rings to all interactive elements
   - Use theme accent color for focus indicators
   - Test with keyboard-only navigation
   - **File:** main.rs lines 6080-6114 (focusable elements)

2. **Context Menu Keyboard Access**
   - Add "Menu" key support (Shift+F10)
   - Implement Escape to close context menus
   - Arrow keys to navigate menu items
   - **Files:** main.rs context menu view methods

3. **Keyboard Shortcuts Help Dialog**
   - Complete show_keyboard_shortcuts_help UI (line 413)
   - Add F1 shortcut to show help
   - Display keyboard shortcut hints in tooltips
   - **File:** main.rs help dialog view

4. **Tab Order Optimization**
   - Ensure logical tab order: Search → Devices → Actions → Settings
   - Add tab stop skip links for screen readers
   - **File:** main.rs get_focusable_elements() method

### Phase 2: File Sharing Enhancements (High Impact, Medium Effort)
**Estimated Effort:** 2 days

#### Tasks
1. **Enhanced Drag-Drop Visual Feedback**
   - Improve drop zone indicator styling (currently minimal at line 4500)
   - Add file count badge when dragging multiple files
   - Show device card highlight when hovering
   - Animate drop zone appearance
   - **File:** main.rs device card view (lines 4495-4531)

2. **Received Files History View**
   - Create dedicated view mode: ViewMode::ReceivedFilesHistory
   - Display received_files_history with timestamps
   - Add "Open" and "Reveal in folder" actions
   - Implement file type icons (method exists at line 3026)
   - Add search/filter for history
   - **File:** main.rs popup_view() method

3. **Transfer Control UI**
   - Add pause/resume buttons to transfer items
   - Implement cancel with confirmation dialog
   - Show estimated time remaining (method exists at line 3077)
   - Display transfer speed
   - **File:** main.rs transfer view sections

4. **Batch File Transfer**
   - Modify SendFiles message handler (line 571)
   - Update file picker to support multiple selection (already partial at line 856)
   - Show queue of pending transfers
   - **File:** main.rs file picker integration

### Phase 3: Screen Share Controls (High Impact, Medium Effort)
**Estimated Effort:** 2 days

#### Tasks
1. **In-Stream Control Overlay Enhancement**
   - Add quality selector dropdown (Low/Medium/High) to overlay
   - Add FPS selector (15/30/60) to overlay
   - Display current quality/fps/audio settings
   - Add tooltips explaining each setting
   - **File:** main.rs screen share overlay view (line 3374)

2. **Audio Toggle Implementation (Backend Required)**
   - Note: Line 2444 TODO indicates backend work needed
   - Add UI toggle in screen share overlay
   - Disable with tooltip if backend unavailable
   - **File:** main.rs, plus potential daemon changes

3. **Switch Source Option**
   - Add "Switch Screen" button to overlay
   - Re-open portal selector dialog
   - Seamless source switching without disconnecting
   - **File:** main.rs screen share control

4. **Connection Status Indicators**
   - Display viewer count prominently (already tracked at line 465)
   - Add connection quality indicator (latency, bandwidth)
   - Show reconnection status
   - **File:** main.rs screen share overlay

### Phase 4: Context Menu Implementation (High Impact, Medium Effort)
**Estimated Effort:** 2 days

#### Tasks
1. **Device Context Menu Completion**
   - Complete device_context_menu_view() implementation
   - Add actions: Connect, Disconnect, Send File, Ring, View Info, Remove, Settings
   - Add keyboard navigation (arrow keys, enter)
   - Right-click activation on device card
   - **File:** main.rs device_context_menu_view()

2. **Transfer Context Menu**
   - Create transfer_context_menu_view() method
   - Actions: Pause, Resume, Cancel, Open File, Reveal in Folder, Copy Path
   - Right-click activation on transfer items
   - **File:** main.rs new method + transfer view integration

3. **MPRIS Context Menu**
   - Create mpris_context_menu_view() method
   - Actions: Track Info, Add to Playlist, Show Player Window, Shuffle, Repeat
   - Right-click on MPRIS player controls
   - **File:** main.rs MPRIS section

4. **Context Menu Component**
   - Create reusable context menu widget following libcosmic patterns
   - Support nested menus if needed
   - Proper positioning (avoid off-screen)
   - Close on click outside or Escape
   - **File:** main.rs or separate component module

### Phase 5: Accessibility & Polish (Medium Impact, Medium Effort)
**Estimated Effort:** 2 days

#### Tasks
1. **ARIA Labels & Semantic Markup**
   - Add descriptive labels to all icon buttons
   - Use semantic HTML/widget roles
   - Ensure logical heading structure
   - **File:** main.rs all view methods

2. **High Contrast Mode Support**
   - Test with high contrast themes
   - Ensure all text meets WCAG contrast ratios
   - Use theme colors consistently
   - **File:** main.rs theme color usage

3. **Touch Target Sizes**
   - Audit all interactive elements
   - Ensure minimum 44px touch targets
   - Add adequate spacing between targets
   - **File:** main.rs button/icon sizing

4. **Keyboard Shortcuts Documentation**
   - Complete keyboard shortcuts help dialog
   - Add inline hints (e.g., "Press ? for help")
   - Document in README
   - **File:** main.rs + README.md

### Phase 6: Workflow Improvements (Low Priority, High Effort)
**Estimated Effort:** 3 days

#### Tasks
1. **Device Pinning UI**
   - Add pin/unpin button to device cards
   - Visual indicator for pinned devices
   - Pinned devices sorted to top
   - Note: Backend already exists (pinned_devices_config at line 434)
   - **File:** main.rs device card view

2. **Notification Preferences**
   - Per-device notification settings UI
   - Toggle categories: Battery, Files, Calls, etc.
   - Global notification preferences
   - **File:** main.rs settings window or device details

3. **First-Run Onboarding**
   - Welcome screen with feature overview
   - Pairing tutorial with steps
   - Permission explanations
   - Note: Onboarding state exists (lines 440-443)
   - **File:** main.rs onboarding view

4. **Quick Connect Dialog**
   - Manual IP/hostname entry form
   - Recent connections list
   - QR code pairing support
   - **File:** main.rs new dialog view

## Code Locations Reference

### Key Files
- **Main Applet:** cosmic-applet-connect/src/main.rs (~6400 lines)
- **DBus Client:** cosmic-applet-connect/src/dbus_client.rs
- **Pinned Devices Config:** cosmic-applet-connect/src/pinned_devices_config.rs
- **Onboarding Config:** cosmic-applet-connect/src/onboarding_config.rs

### Critical Methods & Sections
- **Keyboard Navigation:** Lines 5981-6260
- **Focus Management:** Lines 6080-6260
- **Device Card View:** Lines 4430-4550
- **Screen Share Overlay:** Lines 3372-3470
- **Context Menu Buttons:** Lines 4786-4809
- **Drag-Drop Handling:** Lines 2176-2215, 4495-4531
- **Transfer View:** Lines 3550-3700 (approximate)
- **MPRIS Section:** Lines 3800-4100 (approximate)

### State Variables
- **focus_target:** Line 419
- **drag_hover_device:** Line 421
- **context_menu_device/transfer/mpris:** Lines 424-426
- **active_screen_share:** Line 428
- **active_transfers:** Line 382
- **received_files_history:** Line 383

## Testing Strategy

### Manual Testing Checklist
- [ ] Keyboard-only navigation through all views
- [ ] Drag-drop files onto device cards (single and multiple)
- [ ] Right-click context menus on devices, transfers, MPRIS
- [ ] Screen share quality/fps/audio changes
- [ ] Transfer pause/resume/cancel
- [ ] Received files history view and actions
- [ ] High contrast theme compatibility
- [ ] Touch target sizes on touch-enabled devices

### Automated Tests
- Unit tests for focus navigation logic
- Integration tests for file transfer UI state
- Context menu interaction tests
- Keyboard shortcut handler tests

## Pre-Commit Requirements

Before committing any changes:

1. **COSMIC Code Review**
   ```bash
   @cosmic-code-reviewer /pre-commit-check
   ```
   - Verifies no hard-coded colors/dimensions/radii
   - No `.unwrap()` or `.expect()` calls
   - Proper error handling and logging
   - Theme integration correctness

2. **Code Simplification**
   ```bash
   Run code-simplifier:code-simplifier agent on changes
   ```
   - Code clarity and consistency
   - Better Rust idioms
   - Improved maintainability

## Success Metrics

### User Experience
- 100% keyboard navigation coverage
- Sub-200ms focus transition animations
- Context menu operations < 2 clicks
- File transfer feedback within 100ms
- Zero accessibility violations (axe-core)

### Code Quality
- No hard-coded theme values
- All interactive elements have focus indicators
- Error handling for all user actions
- Comprehensive logging for debugging

## Risk Mitigation

### Known Challenges
1. **Large File Complexity:** main.rs is ~6400 lines
   - **Mitigation:** Incremental changes, thorough testing between phases

2. **libcosmic Widget Limitations:** Some widgets may not support all features
   - **Mitigation:** Research cosmic-toolkit examples, fallback to custom widgets

3. **Backend Dependencies:** Audio toggle requires daemon changes
   - **Mitigation:** UI-only changes first, backend in separate PR

4. **Context Menu Positioning:** Avoiding off-screen menus
   - **Mitigation:** Use libcosmic popup positioning patterns

## Implementation Notes

### COSMIC Design Patterns to Follow
- Use theme colors via `theme_*_color()` functions (lines 54-85)
- Follow spacing scale: SPACE_XXXS to SPACE_XXL (lines 35-41)
- Icon sizes: ICON_XS to ICON_XL (lines 43-51)
- Button styles: Standard, Suggested, Destructive
- Use cosmic::widget components consistently

### Performance Considerations
- Minimize re-renders on focus changes
- Efficient history filtering (MAX_DISPLAYED_HISTORY_ITEMS = 10)
- Debounce search input
- Lazy load album art for MPRIS

## Timeline Estimate

| Phase | Effort | Priority | Dependencies |
|-------|--------|----------|-------------|
| Phase 1: Keyboard Nav | 1 day | High | None |
| Phase 2: File Sharing | 2 days | High | None |
| Phase 3: Screen Share | 2 days | High | Phase 1 |
| Phase 4: Context Menus | 2 days | High | Phase 1 |
| Phase 5: Accessibility | 2 days | Medium | Phase 1-4 |
| Phase 6: Workflow | 3 days | Low | Phase 1-5 |
| **Total** | **12 days** | | |

## Next Steps

1. Review and approve this plan with maintainers
2. Begin Phase 1 implementation (keyboard navigation)
3. Create feature branch: `feature/ui-ux-enhancements-119`
4. Implement phases incrementally with testing
5. Submit PR for review after each phase completion
6. Address feedback and iterate

---

**Document Status:** Draft for Review
**Created:** 2026-02-01
**Issue:** #119
**Assignee:** Claude Sonnet 4.5
