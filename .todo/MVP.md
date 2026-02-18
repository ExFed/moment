# MVP

Features necessary to consider "minimally viable" for production.

- [ ] Static site generation (for GitHub pages)
- [ ] GitHub CI pipeline
- [ ] `build.rs` script that uses `git describe` to determine version
  - add small, unobtrusive version span to page footer for debugging purposes
- [ ] Configurable time limit
  - Parameter in _Play_ view route
- [ ] _Table_ component on the _Play_ view
  - An editable, unordered list of checkbox items for quickly jotting down items to cover once all players have taken a turn (i.e. to "table it" until later)
  - Each _Table_ item consists of:
    - A handle to reorder items
    - A checkbox to "complete" the item (description span is grayed-out and strikethrough when checked)
    - A span with description, clickable to edit description
    - "x" to remove item from list
  - The last item is an empty textbox to add a new _Table_ item (cannot be reordered or removed)
    - Adds new item if "enter" key is pressed with 1 or more characters in textbox
- [ ] _Lobby_ view
  - The default route where the host can...
    - Configure the time limit
    - _Start_ button (link to the _Play_ view with configured time limit)
