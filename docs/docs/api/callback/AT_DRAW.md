# ModCallbacks.AT_DRAW

!!! info "回调函数签名"

    ```lua
    fun(g: Graphics)
    ```

    参数:

    - `g` [Graphics](../class/Graphics.md): 图形类

在游戏渲染的每一帧，此回调会被触发。

尽可能避免在此回调中更新逻辑，逻辑帧与渲染帧的速率并不一致。
