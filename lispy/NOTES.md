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
