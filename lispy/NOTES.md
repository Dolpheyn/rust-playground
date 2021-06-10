# Notes

## On Creating an Interactive Prompt

### Raw terminals

In raw mode, every terminal events like key presses, mouse movements, and resize
is "captured" and it is up to the dev how he wants to handle it.

If you write anything, nothing will show up on the screen. You will have to
handle character key presses and print it to the screen, and push it in a
buffer.

Even back space does not work like how you would expect. You would have to move
your cursor back 1 time, write a space (" ") to overwrite the last character and
move the cursor left again. At least that's how I did it xD.

### Moving Cursors

I'm still trying to figure out how to not let the cursor move past the prompt on
the left side or past the buffer on the right side. One way I could think of is
tracking the position of the cursor relative to some left and right "walls".

e.g.

```shell
lispy > _
```

We can say the cursor `_` is at position 0 and our buffer is empty. If
LeftArrow or RightArrow is pressed, do nothing. The cursor is at the end of both
"walls".

```shell
lispy > some buffer_
```

We can say that this cursor `_` is at position (buffer len + 1). Here, the
buffer len is 11. If RightArrow is pressed, do nothing. But it can move to the
left. The cursor is right beside the right "wall".

```shell
lispy > some _uffer
```

Here the cursor can move freely to both sides, because its in between
0..(buffer_len + 1) which is our left and right walls.

---

### Logs

2021-06-10

I [implemented] the moving cursor, and editing mid-buffer mechanism!

The "walls" idea worked, and I named the left and right walls
`start_of_buffer_pos` and `end_of_buffer_pos` respectively. I also created a
variable to track the current cursor position—`cursor_pos`.

The moving part was pretty easy compared to editing mid-buffer. Update
`cursor_pos` everytime movement is made (Left/Right/Home/End is pressed). If
`cursor_pos` is at `start_of_buffer_pos`, don't move to the left. If it is at
`end_of_buffer_pos`, don't move to the right.

Editing mid-buffer was a bit tricky. First, you need to separate the "normal"
and mid-buffer edits. e.g.:

For backspace and writing characters on the screen, normal edit is at the end of
the line.

```rust
if cursor_pos == end_of_buffer_pos {
  /** normal implementation **/
} else {
  /** mid or start of buffer implementation **/
}
```

For backspace, start of buffer scenario is handled at the beginning of the case:

```rust
if cursor_pos == start_of_buffer_pos {
  continue;
}
```

So we don't have to worry for that.

And here's the annotated `else` part for inserting chars:

```rust
if cursor_pos == end_of_buffer_pos {
  /** normal implementation **/
} else {
  // To get the index to insert to, we subtract the start position from the
  // current position.
  //
  // Let's say we already wrote "hello",
  //
  // `lispy > hello`
  //
  // And our cursor is at the second l
  //
  // `lispy > hel_o`
  //          ^  ^
  //  start __|  |__ current
  //  (9)            (12)
  //
  // So our prompt starts at the 9th column and the cursor is at 12th. We
  // will be inserting at index `12 - 9 = 3`
  let insert_idx = (cursor_pos - start_of_buffer_pos) as usize;

  // Now we use String::insert in Rust. If we pressed "x" from the example
  // above, our buffer would now be "helxlo".
  buffer.insert(insert_idx, c);

  // So here's the tricky part that I spent ~10 mins on. This had to be way more
  // complicated if crossterm don't have the `SavePosition` and `RestorePosition`
  // commands. Thank you for that crossterm, top tier crate.
  //
  // Remember that we haven't wrote anything on the screen yet. And cursor's on
  // the second l.
  //
  // `lispy > hel_o`
  //
  // First we save that position, and print whatever is in our buffer from the
  // insert position to the end (this will cause problems for non utf-8 chars
  // due to the byte size differences, but I don't want to deal with that because
  // we're only implementing a lisp repl here).
  //
  // Anything on our current cursor and beyond that will be overwritten. So the
  // outcome:
  //
  // `lispy > helxlo_`
  //
  // Now our cursor is at the end of the line because it will be on the right of
  // whatever char it last printed. And this is where that `SavePosition` comes
  // in handy because we can avoid doing more math to figure out how many
  // columns we have to move back. Use `RestorePosition` and we're back to where
  // we were
  //
  // `lispy > hel_lo`
  //
  // Lastly, move 1 column to the right to make it how we expect it to behave.
  //
  // `lispy > helx_o`
  stdout
    .queue(SavePosition)?
    .queue(Print(&buffer[insert_idx..]))?
    .queue(RestorePosition)?
    .queue(MoveRight(1))?
}

// Increase cursor position since we move 1 to the right, and increase the end
// of buffer position since we added 1 char to the line.
cursor_pos += 1;
end_of_buffer_pos += 1;

// Flush whatever we had queued in the stdout.
stdout.flush()?;
```

[implemented]: https://github.com/Dolpheyn/rust-practice/commit/f2facd478007d8880bb4a44a67de563232570c99
