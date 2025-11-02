This is my successful port of a table filter control in Rust for the egui library. 

I first made this for [JavaFX almost 10 years ago](http://fxexperience.com/2016/03/introducing-the-controlsfx-tablefilter/). 

I would like to make this into a library so I welcome any contributions to make the API more streamlined. It is highly flexible and configurable, and I'd like to expand on that while keeping it easy to use. 

<img width="640" height="546" alt="image" src="https://github.com/user-attachments/assets/2a8a7bbf-5757-4758-95d6-5e2b73aa29a8" />

You can also create custom search functionality to parse the strings for special syntax, like regular expressions or date ranges. 

<img width="586" height="593" alt="image" src="https://github.com/user-attachments/assets/3e6a93cc-b93d-4841-97f5-41d20eb31ed4" />

## TODO

- [ ] Gray out entries that are no longer visible due to other column filter
- [ ] Have pre-defined templates to streamline common data types and operations (e.g., int ranges, date ranges, regular expressions)\
- [ ] Add double-clicking sorting functionality on the headers
- [ ] Document a checklist that is needed for successful application to a table
- [ ] Explore patterns outside the table in egui-extras, as this control is agnostic to controls
- [ ] Find opportunities to make more idiomatic without inflicting too much bias
