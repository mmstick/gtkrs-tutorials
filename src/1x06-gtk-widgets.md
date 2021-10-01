# GTK Widget Reference

List of roughly every widget included in GTK

## Containers

Containers control the behavior and layout of the widgets within them.

-   [AspectFrame](https://gtk-rs.org/gtk3-rs/stable/latest/docs/gtk/struct.AspectFrame.html): Ensures that the widget retains the same aspect when resized
-   [Box](https://gtk-rs.org/gtk3-rs/stable/latest/docs/gtk/struct.Box.html): Lays widgets in vertical or horizontal layouts
-   [ButtonBox](https://gtk-rs.org/gtk3-rs/stable/latest/docs/gtk/struct.ButtonBox.html): Arranges buttons within the container, and themes may style buttons packed this way in a nicer way
-   [Expander](https://gtk-rs.org/gtk3-rs/stable/latest/docs/gtk/struct.Expander.html): Shows/hides a widget with a button that expands to reveal a hidden widget
-   [FlowBox](https://gtk-rs.org/gtk3-rs/stable/latest/docs/gtk/struct.FlowBox.html): Lays widgets horizontally, and dynamically shifts them to new rows as the parent container shrinks
-   [Frame](https://gtk-rs.org/gtk3-rs/stable/latest/docs/gtk/struct.Frame.html): Displays a frame around a widget
-   [Grid](https://gtk-rs.org/gtk3-rs/stable/latest/docs/gtk/struct.Grid.html): Lays widgets within a grid of rows and columns, with each widget occupying a X,Y position with a defined width and height
-   [HeaderBar](https://gtk-rs.org/gtk3-rs/stable/latest/docs/gtk/struct.HeaderBar.html): Replaces the title bar, where widgets can be packed from the start, the center, or the end of the bar
-   [Notebook](https://gtk-rs.org/gtk3-rs/stable/latest/docs/gtk/struct.Notebook.html): Identical to a stack, but has tabs for switching between widgets. Essentially a Stack + StackSwitcher with a set style
-   [Paned](https://gtk-rs.org/gtk3-rs/stable/latest/docs/gtk/struct.Paned.html): Containers two widgets side-by-side with a boundary between them that allows the user to resize between them
-   [Revealer](https://gtk-rs.org/gtk3-rs/stable/latest/docs/gtk/struct.Revealer.html): Conceals and reveals a widget with an animation
-   [ScrolledWindow](https://gtk-rs.org/gtk3-rs/stable/latest/docs/gtk/struct.ScrolledWindow.html): Makes the contained widget scrollable
-   [Stack](https://gtk-rs.org/gtk3-rs/stable/latest/docs/gtk/struct.Stack.html): Stores multiple widgets, but only one widget is shown at a time. May be combined with a StackSwitcher to have tabs
-   [Toolbar](https://gtk-rs.org/gtk3-rs/stable/latest/docs/gtk/struct.Toolbar.html): Bar at the top of the window for containing tool items

## Lists

Containers with selectable widgets

-   [ComboBox](https://gtk-rs.org/gtk3-rs/stable/latest/docs/gtk/struct.ComboBox.html): Used in conjuction with a tree model to show a list of options to select from
-   [ComboBoxText](https://gtk-rs.org/gtk3-rs/stable/latest/docs/gtk/struct.ComboBoxText.html): Streamlined variant of a ComboBox to choose from a list of text options
-   [IconView](https://gtk-rs.org/gtk3-rs/stable/latest/docs/gtk/struct.IconView.html): Think of a file browser with mouse drag selections. Essentially a FlowBox-like container with a grid of icons with text
-   [ListBox](https://gtk-rs.org/gtk3-rs/stable/latest/docs/gtk/struct.ListBox.html): Each widget is an interactive row in a list, which may be activated or clicked, and may support multiple selections
-   [TreeView](https://gtk-rs.org/gtk3-rs/stable/latest/docs/gtk/struct.TreeView.html): Used to present tabular data, with each row being an object in the list, and each column a field of that object

## Text

Containers which display or receive text

-   [Label](https://gtk-rs.org/gtk3-rs/stable/latest/docs/gtk/struct.Label.html): Displays text without any ability to copy or edit the text
-   [Entry](https://gtk-rs.org/gtk3-rs/stable/latest/docs/gtk/struct.Entry.html): Text box for a single line of text
-   [TextView](https://gtk-rs.org/gtk3-rs/stable/latest/docs/gtk/struct.TextView.html): Multi-line text box
-   [SearchEntry](https://gtk-rs.org/gtk3-rs/stable/latest/docs/gtk/struct.SearchEntry.html): Entry designed for use for searches
-   [SearchBar](https://gtk-rs.org/gtk3-rs/stable/latest/docs/gtk/struct.SearchBar.html): Toolbar that reveals a search entry when the user starts typing

## Buttons

Widgets that can be clicked or activated by keyboard

-   [AppChooserButton](https://gtk-rs.org/gtk3-rs/stable/latest/docs/gtk/struct.AppChooserButton.html): Button that shows an app chooser dialog
-   [Button](https://gtk-rs.org/gtk3-rs/stable/latest/docs/gtk/struct.Button.html): Interactive widget that may contain text, an image, or other widgets
-   [CheckButton](https://gtk-rs.org/gtk3-rs/stable/latest/docs/gtk/struct.CheckButton.html): Check mark with a label that can be toggled on/off
-   [ColorButton](https://gtk-rs.org/gtk3-rs/stable/latest/docs/gtk/struct.ColorButton.html): Displays a color and shows a color chooser dialog to select a different one
-   [FileChooserButton](https://gtk-rs.org/gtk3-rs/stable/latest/docs/gtk/struct.FileChooserButton.html): Shows a file chooser dialog to select file(s) or folder(s)
-   [FontButton](https://gtk-rs.org/gtk3-rs/stable/latest/docs/gtk/struct.FontButton.html): Displays a font and shows a font chooser dialog ot select a different one
-   [LinkButton](https://gtk-rs.org/gtk3-rs/stable/latest/docs/gtk/struct.LinkButton.html): Hyperlink text button for linking to a URI
-   [LockButton](https://gtk-rs.org/gtk3-rs/stable/latest/docs/gtk/struct.LockButton.html): Button with a lock icon for unlocking / locking privileged options
-   [MenuButton](https://gtk-rs.org/gtk3-rs/stable/latest/docs/gtk/struct.MenuButton.html): Button for showing a popover menu on click
-   [RadioButton](https://gtk-rs.org/gtk3-rs/stable/latest/docs/gtk/struct.RadioButton.html): When grouped with other radio buttons, only one button may be activate
-   [ScaleButton](https://gtk-rs.org/gtk3-rs/stable/latest/docs/gtk/struct.ScaleButton.html): Button that pops up a scale
-   [SpinButton](https://gtk-rs.org/gtk3-rs/stable/latest/docs/gtk/struct.SpinButton.html): Number entry with buttons for incrementing and decrementing
-   [StackSidebar](https://gtk-rs.org/gtk3-rs/stable/latest/docs/gtk/struct.StackSidebar.html): Vertical tabs for a stack
-   [StackSwitcher](https://gtk-rs.org/gtk3-rs/stable/latest/docs/gtk/struct.StackSwitcher.html): Horizontal tabs for a stack
-   [Switch](https://gtk-rs.org/gtk3-rs/stable/latest/docs/gtk/struct.Switch.html): Toggle button represented as an off/on switch
-   [ToggleButton](https://gtk-rs.org/gtk3-rs/stable/latest/docs/gtk/struct.ToggleButton.html): Button that toggles between being pressed in and unpressed
-   [VolumeButton](https://gtk-rs.org/gtk3-rs/stable/latest/docs/gtk/struct.VolumeButton.html): Button that pops up a volume scale

## Display

Widgets that display things

-   [DrawingArea](https://gtk-rs.org/gtk3-rs/stable/latest/docs/gtk/struct.DrawingArea.html): Provides a canvas for drawing images onto
-   [EventBox](https://gtk-rs.org/gtk3-rs/stable/latest/docs/gtk/struct.EventBox.html): Makes it possible for a widget to receive button / mouse events
-   [GLArea](https://gtk-rs.org/gtk3-rs/stable/latest/docs/gtk/struct.GLArea.html): Context for rendering OpenGL onto
-   [Image](https://gtk-rs.org/gtk3-rs/stable/latest/docs/gtk/struct.Image.html): Displays a picture
-   [InfoBar](https://gtk-rs.org/gtk3-rs/stable/latest/docs/gtk/struct.InfoBar.html): Hidden bar that is revealed when info or an error is to be shown
-   [LevelBar](https://gtk-rs.org/gtk3-rs/stable/latest/docs/gtk/struct.LevelBar.html): Shows a level of a scale
-   [ProgressBar](https://gtk-rs.org/gtk3-rs/stable/latest/docs/gtk/struct.ProgressBar.html): Shows a progress bar
-   [Separator](https://gtk-rs.org/gtk3-rs/stable/latest/docs/gtk/struct.Separator.html): Shows a horizontal or vertical separator
-   [ShortcutLabel](https://gtk-rs.org/gtk3-rs/stable/latest/docs/gtk/struct.ShortcutLabel.html): Keyboard shortcut label
-   [Spinner](https://gtk-rs.org/gtk3-rs/stable/latest/docs/gtk/struct.Spinner.html): Shows a spinning animation
-   [Statusbar](https://gtk-rs.org/gtk3-rs/stable/latest/docs/gtk/struct.Statusbar.html): Displays information at the bottom of the window

## Misc

Everything else

-   [PlacesSidebar](https://gtk-rs.org/gtk3-rs/stable/latest/docs/gtk/struct.PlacesSidebar.html): Displays frequently visited places in the file system
-   [Plug](https://gtk-rs.org/gtk3-rs/stable/latest/docs/gtk/struct.Plug.html) / [Socket](https://gtk-rs.org/gtk3-rs/stable/latest/docs/gtk/struct.Socket.html): Allows sharing widgets across windows

> An exhaustive list of the gtk widgets can be found in the [widget gallery](https://docs.gtk.org/gtk4/visual_index.html), but the API version is GTK 4 and above, so crosscheck with the GTK3 docs to get the correct syntax for a widget
