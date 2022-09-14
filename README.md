# rubil

Build HTML without writing HTML. A small project I want to work on for fun. Are you fed up with breaking your fingers when
writing brackets, equal signs and back slashes like
```html
<div class="container">
	<ul>
		<li>Owie</li>
		<li>my</li>
		<li>fingers.</li>
	</ul>
</div>
```
you can write your HTML without ever opening a bracket with `rubil`! For example the above turns into
```rust
let html: String = Div::new()
	.class("container")
		.ul(|ul| ul
			.li("Owie")
			.li("my")
			.li("fingers"))
	.build;
```