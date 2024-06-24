---
layout: post
title: Formatting Markdown with Comrak
---

# Title
## Section 1
### Section 1.2
#### Section 1.3
##### Section 1.4
###### Section 1.5

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Fusce et erat eget mauris semper aliquam in non nulla. Quisque a velit semper, varius lacus at, egestas ante. Morbi aliquam viverra elit eu volutpat. 

[http://example.link](http://example.link)

```md
# Title
## Section 1
### Section 1.2
#### Section 1.3
##### Section 1.4
###### Section 1.5

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Fusce et erat eget mauris semper aliquam in non nulla. Quisque a velit semper, varius lacus at, egestas ante. Morbi aliquam viverra elit eu volutpat. 

[http://example.link](http://example.link)
```

## Quotes
> Ut posuere tincidunt tellus, non volutpat mi sagittis eget. 
```md
> Ut posuere tincidunt tellus, non volutpat mi sagittis eget. 
```

## Autolink
https://autolink.link
```md
https://autolink.link
```

## Description lists
First term

: Details for the **first term**

Second term

: Details for the **second term**

    More details in second paragraph.

```md
First term

: Details for the **first term**

Second term

: Details for the **second term**

    More details in second paragraph.

```

## Math code
```math
x^2
```
``````markdown
Inline math $`1 + 2`$

```math
x^2
```
``````
## Math dolars
Inline math $`3^2 + 4^2 = 5^2`$

$$ 
\frac{dy}{dt}=f(t,y),\quad y(t_{0})=y_{0}.
$$
```md
Inline math $`3^2 + 4^2 = 5^2`$

$$ 
\frac{dy}{dt}=f(t,y),\quad y(t_{0})=y_{0}.
$$
```

## Multiline block quotes
>>>
Multiline block quotes
- one
- two
>>>
```
>>>
Multiline block quotes
- one
- two
>>>
```

## Shortcodes
Happy Friday! :smile:
```md
Happy Friday! :smile:
```

## Strikethrough
Hello ~world~ there.
```md
Hello ~world~ there.
```

## Superscript
e = mc^2^.
```md
e = mc^2^.
```

## Tasklist
- [ ] task 1
- [ ] task 2
- [x] task 3
```md
- [ ] task 1
- [ ] task 2
- [x] task 3
```

## Images
![image](https://www.baidu.com/img/doodlegaokaokaohou_5b0f886d182bfd1761854cb4dfa72fa8.gif)
```md
![image](https://www.baidu.com/img/doodlegaokaokaohou_5b0f886d182bfd1761854cb4dfa72fa8.gif)
```

## Table
| Column 1 | Column 2 | Column 3 |
| :- | :-: | -: | 
| **entry 1** | data 1 | true,<br>but check it |
| **entry 2** | data 2  | false |
```md
| Column 1 | Column 2 | Column 3 |
| :- | :-: | -: | 
| **entry 1** | data 1 | true,<br>but check it |
| **entry 2** | data 2  | false |
```


## Definition lists

**Fact 1**
: Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia Curae; 

**Fact 2**
: Etiam in vulputate orci. Proin sit amet urna augue. Cras nisl arcu, ullamcorper sollicitudin dui a, imperdiet lobortis libero.

```md
**Fact 1**
: Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia Curae; 

**Fact 2**
: Etiam in vulputate orci. Proin sit amet urna augue. Cras nisl arcu, ullamcorper sollicitudin dui a, imperdiet lobortis libero.
```

## Lists

### Ordered lists
1. numbered list entry 1
2. numbered list entry 2
	1. numbered list entry 1
	2. numbered list entry 2
	* mixture 
3. numbered list entry 3
```md
1. numbered list entry 1
2. numbered list entry 2
	1. numbered list entry 1
	2. numbered list entry 2
	* mixture 
3. numbered list entry 3
```

### Unordered lists
* unordered list entry 1
* unordered list entry 2
	* unordered list entry 1
	* unordered list entry 2
	1. mixture
	2. mixture
* unordered list entry 3
```md
* unordered list entry 1
* unordered list entry 2
	* unordered list entry 1
	* unordered list entry 2
	1. mixture
	2. mixture
* unordered list entry 3
```

## Syntax Highlighting (Fenced code blocks)
 
Python:
```python
import numpy as np
a=np.arange(5)
print(a+a)
```

Javascript:
```java
function fancyAlert(arg) {
  if(arg) {
    $.facebox({div:'#foo'})
  }
}
```

## Footnotes
Donec[^1] feugiat fringilla nulla, vitae egestas enim[^1][^2] iaculis at. 

[^1]: Nulla nec tincidunt justo. 
[^2]: Donec condimentum dolor est, ut molestie urna luctus in. 