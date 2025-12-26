# ModCallbacks.AT_BOARD_MOUSE_UP

!!! info "回调函数签名"

    ```lua
    fun(mousecode: MouseCode, pos: Vec2): boolean?
    ```

    参数:

    - `mousecode` [MouseCode](../alias/MouseCode.md): 松开的鼠标点击代码
    - `pos` [Vec2](../class/Vec2.md): 鼠标松开的坐标

    返回值:

    - 如果是 `true` 则会消费此次事件

在游戏内关卡游玩中鼠标点击松开时会触发此回调。松开的按键由鼠标键码表示。

同时松开多个键**不会**多次触发此回调函数，而是用特殊的鼠标键码表示。如果是双击事件之后的松开，只会触发相应的单击鼠标键码。

如果返回 `true`，则优先级低于此 Mod 的其他 Mod 将不会触发此回调，游戏也不会响应此次事件。
