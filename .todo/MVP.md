# MVP

Features necessary to consider "minimally viable" for production.

- [ ] Static site generation (e.g. for GitHub pages)
- [ ] GitHub CI pipeline
- [ ] A small, unobtrusive span on page footer with the application version (via `DESCRIBE_VERSION`)
- [ ] Configurable time limit
  - Parameter in _Play_ view route
- [ ] _Table_ component on the _Play_ view
  - An editable, unordered list of checkbox items for quickly jotting down items to cover once all players have taken a turn (i.e. to "table it" until later)
  - Each _Table_ item consists of, from left-to-right:
    - A handle to reorder items
    - A checkbox to "complete" the item (description span is grayed-out and strikethrough when checked)
    - A span with description; click span to edit (replace span with textbox); user presses "enter" or leaves focus to commit edit (must have at least one character in textbox, otherwise cancels edit), user presses "escape" to cancel edit; once edit is committed or cancelled, replace textbox with span
    - An "x" symbol to remove item from list
  - The last item is an empty textbox to add a new _Table_ item (cannot be reordered or removed)
    - Adds new item if "enter" key is pressed while 1 or more characters in textbox
- [ ] _Lobby_ view
  - The default route where the host can...
    - Configure the time limit
    - _Start_ button (link to the _Play_ view with configured time limit)
