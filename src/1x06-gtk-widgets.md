# GTK Widget Reference

List of roughly every widget included in GTK

## Containers

Containers control the behavior and layout of the widgets within them.

- [AspectFrame](): Ensures that the widget retains the same aspect when resized
- [Box](): Lays widgets in vertical or horizontal layouts
- [ButtonBox](): Arranges buttons within the container, and themes may style buttons packed this way in a nicer way
- [Expander](): Shows/hides a widget with a button that expands to reveal a hidden widget
- [FlowBox](): Lays widgets horizontally, and dynamically shifts them to new rows as the parent container shrinks
- [Frame](): Displays a frame around a widget
- [Grid](): Lays widgets within a grid of rows and columns, with each widget occupying a X,Y position with a defined width and height
- [HeaderBar](): Replaces the title bar, where widgets can be packed from the start, the center, or the end of the bar
- [Notebook](): Identical to a stack, but has tabs for switching between widgets. Essentially a Stack + StackSwitcher with a set style
- [Paned](): Containers two widgets side-by-side with a boundary between them that allows the user to resize between them
- [Revealer](): Conceals and reveals a widget with an animation
- [ScrolledWindow](): Makes the contained widget scrollable
- [Stack](): Stores multiple widgets, but only one widget is shown at a time. May be combined with a StackSwitcher to have tabs
- [Toolbar](): Bar at the top of the window for containing tool items

## Lists

Containers with selectable widgets

- [ComboBox](): Used in conjunction with a tree model to show a list of options to select from
- [ComboBoxText](): Streamlined variant of a ComboBox to choose from a list of text options
- [IconView](): Think of a file browser with mouse drag selections. Essentially a FlowBox-like container with a grid of icons with text
- [ListBox](): Each widget is an interactive row in a list, which may be activated or clicked, and may support multiple selections
- [TreeView](): Used to present tabular data, with each row being an object in the list, and each column a field of that object

## Text

Containers which display or receive text

- [Label](): Displays text without any ability to copy or edit the text
- [Entry](): Text box for a single line of text
- [TextView](): Multi-line text box
- [SearchEntry](): Entry designed for use for searches
- [SearchBar](): Toolbar that reveals a search entry when the user starts typing

## Buttons

Widgets that can be clicked or activated by keyboard

- [AppChooserButton](): Button that shows an app chooser dialog
- [Button](): Interactive widget that may contain text, an image, or other widgets
- [CheckButton](): Check mark with a label that can be toggled on/off
- [ColorButton](): Displays a color and shows a color chooser dialog to select a different one
- [FileChooserButton](): Shows a file chooser dialog to select file(s) or folder(s)
- [FontButton](): Displays a font and shows a font chooser dialog ot select a different one
- [LinkButton](): Hyperlink text button for linking to a URI
- [LockButton](): Button with a lock icon for unlocking / locking privileged options
- [MenuButton](): Button for showing a popover menu on click
- [RadioButton](): When grouped with other radio buttons, only one button may be activate
- [ScaleButton](): Button that pops up a scale
- [SpinButton](): Number entry with buttons for incrementing and decrementing
- [StackSidebar](): Vertical tabs for a stack
- [StackSwitcher](): Horizontal tabs for a stack
- [Switch](): Toggle button represented as an off/on switch
- [ToggleButton](): Button that toggles between being pressed in and unpressed
- [VolumeButton](): Button that pops up a volume scale

## Display

Widgets that display things

- [DrawingArea](): Provides a canvas for drawing images onto
- [EventBox](): Makes it possible for a widget to receive button / mouse events
- [GLArea](): Context for rendering OpenGL onto
- [Image](): Displays a picture
- [InfoBar](): Hidden bar that is revealed when info or an error is to be shown
- [LevelBar](): Shows a level of a scale
- [ProgressBar](): Shows a progress bar
- [Separator](): Shows a horizontal or vertical separator
- [ShortcutLabel](): Keyboard shortcut label
- [Spinner](): Shows a spinning animation
- [Statusbar](): Displays information at the bottom of the window

## Misc

Everything else

- [PlacesSidebar](): The Places sidebar of a file browser
- [Plug]() / [Socket](): Allows sharing widgets across windows