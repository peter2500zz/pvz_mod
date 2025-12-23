# Vec2

!!! info

    本文部分内容由 AI 辅助编写，如果发现错误请提交 issue。

Vec2 是 Rumia 内置的二维向量类。一般用于存储一组坐标或者长宽。Vec2 同时也提供了基础的向量运算。

## 字段

以下是 Vec2 的所有字段:

---

### x

类型:

- number

表示 x 的浮点数。

---

### y

类型:

- number

表示 y 的浮点数。

---

### New

函数签名:

```lua
fun(x: number, y: number): Vec2
```

参数:

- `x` number: x 的值。
- `y` number: y 的值。

返回值:

- 由 x 和 y 创建的 Vec2 类。

给定 x 与 y，创建一个 Vec2 以便于计算。

---

### Zero

函数签名:

```lua
fun(): Vec2
```

返回值:

- 一个 Vec2 零向量。

创建一个零向量。

---

## 方法

以下是 Vec2 的所有方法:

---

### Add

函数签名:

```lua
fun(self, other: Vec2): Vec2
```

参数:

- `other` Vec2: 另一个 Vec2 类。

返回值:

- 向量加法后的结果。

以自身和另一个向量进行向量加法，返回计算的结果，不会修改自身。

---

### Sub

函数签名:

```lua
fun(self, other: Vec2): Vec2
```

参数:

- `other` Vec2: 另一个 Vec2 类。

返回值:

- 向量减法后的结果。

以自身减去另一个向量，返回计算的结果，不会修改自身。

---

### Mul

函数签名:

```lua
fun(self, scalar: number): Vec2
```

参数:

- `scalar` number: 标量值。

返回值:

- 向量与标量相乘后的结果。

对向量的每个分量进行标量乘法，返回新的向量，不会修改自身。

---

### Div

函数签名:

```lua
fun(self, scalar: number): Vec2
```

参数:

- `scalar` number: 标量值。

返回值:

- 向量与标量相除后的结果。

对向量的每个分量进行标量除法，返回新的向量，不会修改自身。

---

### Dot

函数签名:

```lua
fun(self, other: Vec2): number
```

参数:

- `other` Vec2: 另一个 Vec2 类。

返回值:

- 两个向量的点积结果。

计算当前向量与另一个向量的点积（内积），不会修改自身。

---

### Length

函数签名:

```lua
fun(self): number
```

返回值:

- 向量的长度（模）。

计算并返回向量的长度。

---

### LengthSquared

函数签名:

```lua
fun(self): number
```

返回值:

- 向量长度的平方。

计算并返回向量长度的平方，避免开平方运算。

---

### Normalize

函数签名:

```lua
fun(self): Vec2
```

返回值:

- 归一化后的单位向量。

将向量归一化，返回方向相同、长度为 1 的新向量，不会修改自身。

---

### Distance

函数签名:

```lua
fun(self, other: Vec2): number
```

参数:

- `other` Vec2: 另一个 Vec2 类。

返回值:

- 两点之间的距离。

计算当前向量表示的点与另一点之间的距离。

---

### DistanceSquared

函数签名:

```lua
fun(self, other: Vec2): number
```

参数:

- `other` Vec2: 另一个 Vec2 类。

返回值:

- 两点之间距离的平方。

计算当前向量表示的点与另一点之间距离的平方，避免开平方运算。

---

### Lerp

函数签名:

```lua
fun(self, other: Vec2, t: number): Vec2
```

参数:

- `other` Vec2: 目标向量。
- `t` number: 插值系数，通常在 0~1 之间。

返回值:

- 插值后的向量结果。

在当前向量与目标向量之间进行线性插值，返回新的向量，不会修改自身。

---

### Angle

函数签名:

```lua
fun(self): number
```

返回值:

- 向量的角度（弧度）。

计算向量相对于 X 轴正方向的角度，结果以弧度表示。

---

### AngleTo

函数签名:

```lua
fun(self, other: Vec2): number
```

参数:

- `other` Vec2: 目标向量。

返回值:

- 从当前点指向目标点的角度（弧度）。

计算从当前向量指向另一个向量的方向角度。

---

### Rotate

函数签名:

```lua
fun(self, angle: number): Vec2
```

参数:

- `angle` number: 旋转角度（弧度）。

返回值:

- 旋转后的向量结果。

将向量按给定弧度进行旋转，返回新的向量，不会修改自身。

---

### Cross

函数签名:

```lua
fun(self, other: Vec2): number
```

参数:

- `other` Vec2: 另一个 Vec2 类。

返回值:

- 叉积结果（标量）。

计算二维向量的叉积，结果为标量值。

---

### Reflect

函数签名:

```lua
fun(self, normal: Vec2): Vec2
```

参数:

- `normal` Vec2: 用于反射的法线向量。

返回值:

- 反射后的向量结果。

根据给定的法线向量对当前向量进行反射计算，返回新的向量，不会修改自身。
