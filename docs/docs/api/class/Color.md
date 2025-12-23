# Color

Color 是 Rumia 内置的颜色类，用于创建或者保存一个 RGBA 色值。

## 字段

以下是 Color 的所有字段:

---

### New

函数签名:

```lua
fun(red: integer, green: integer, blue: integer, alpha: integer): Color
```

参数:

- `red` integer: 红色的色值。
- `green` integer: 绿色的色值。
- `blue` integer: 蓝色的色值。
- `alpha` integer: 透明度的色值。

返回值:

- 由这四个色值创建的 Color。

通过四个颜色的色值创建一个 Color 类。

---

### red

类型:

- integer

红色的色值。

0~255以外的值无效。

---

### green

类型:

- integer

绿色的色值。

0~255以外的值无效。

---

### blue

类型:

- integer

蓝色的色值。

0~255以外的值无效。

---

### alpha

类型:

- integer

透明度的色值。

0~255以外的值无效。

