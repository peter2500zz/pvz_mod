# LawnApp

LawmApp 是个代表了游戏的类，几乎其他所有东西都是 LawnApp 的字段。

LawnApp 常用于获取游戏内其他的类，比如典型的 Board 类。

!!! info

    不建议保存 LawnApp。除非必要，否则请尽可能使用 [GetLawnApp](../global/GetLawnApp.md) 来获取 LawnApp

## 方法

以下是 LawnApp 的所有方法:

---

### GetWindowSize

获取游戏窗口的尺寸。

函数签名:

```lua
fun(self): Vec2
```

返回值:

- 一个包含长度与高度的 [Vec2](../class/Vec2.md)

---

### GetMousePos

获取鼠标在游戏窗口内的坐标。

函数签名:

```lua
fun(self): Vec2
```

返回值:

- 一个包含 x坐标 和 y坐标 的 [Vec2](../class/Vec2.md)

---

### GetBoard

获取游戏关卡类，如果游戏还没有进入关卡，则返回 nil。

函数签名:

```lua
fun(self): Board?
```

返回值:

- 游戏类 [Board](../class/Board.md) 或 nil
