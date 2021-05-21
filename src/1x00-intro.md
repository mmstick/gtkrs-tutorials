# Basics

> This tutorial series is currently a work in progress. [GitHub](https://github.com/mmstick/gtkrs-tutorials)

The purpose of this tutorial is to demonstrate GTK application development in Rust from an event-drive perspective. After gaining a lot of experience, I have come to the conclusion that this is the best way to develop GTK applications, and through this tutorial I will share what I consider to be best practices.

Besides the first chapter, each chapter will contain a useful application that you will develop alongside the tutorial. Through this, you will gain some insight into how these applications are developed, and gain experience with a variety of aspects that GTK and Rust have to offer to an application developer.

You will learn the following:

- First and foremost, GLib and GTK
- How to navigate the GTK-rs API documentation
- Using GLib's global and thread-local async executors
- Creating and configuring widgets with cascade macro
- APP IDs and preventing applications from launching multiple instances
- Adding an event handler in a GTK application
- Utilizing an entity-component approach to widget management
- Adhering to XDG standards
- Embedding resources into a GTK applications
- Translating applications with Fluent
- Packaging for Debian platforms
- GNOME Human Interface Guidelines
