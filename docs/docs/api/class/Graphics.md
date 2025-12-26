# Graphics

!!! info

    本文部分内容由 AI 辅助编写，如果发现错误请提交 issue。

Graphics 是用于绘制图形的接口定义。

## 方法

以下是 Graphics 的所有方法:

---

### SetLayer

设置后续绘制操作所使用的渲染层。

不同的渲染层决定了绘制内容在画面中的叠放顺序，相同的渲染层受到 Mod 加载顺序的影响。后加载的 Mod 将后绘制。

函数签名:

```lua
fun(self, layer: RenderLayer)
```

参数:

- `layer` RenderLayer: 要使用的渲染层。

---

### SetColor

设置后续绘制操作所使用的颜色。

函数签名:

```lua
fun(self, color: Color)
```

参数:

- `color` Color: 绘制所使用的颜色。

---

### DrawRect

绘制一个空心矩形，仅绘制矩形的边框。

函数签名:

```lua
fun(self, rect: Rect2)
```

参数:

- `rect` [Rect2](../class/Rect2.md): 要绘制的矩形区域。

---

### FillRect

绘制一个实心矩形，填充整个矩形区域。

函数签名:

```lua
fun(self, rect: Rect2)
```

参数:

- `rect` [Rect2](../class/Rect2.md): 要绘制的矩形区域。
