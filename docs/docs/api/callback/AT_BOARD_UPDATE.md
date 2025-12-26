# ModCallbacks.AT_BOARD_UPDATE

!!! info "回调函数签名"

    ```lua
    fun(delta: number)
    ```

    参数:

    - `delta` number: 距离上一逻辑帧过去的时间（秒）。

游戏内每一逻辑帧运算完毕都会调用此回调。
