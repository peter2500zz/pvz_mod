# ModCallbacks.AT_ZOMBIE_DIE

!!! info "回调函数签名"

    ```lua
    fun(zombie: Zombie)
    ```

    参数:

    - `zombie` Zombie: 将要死亡的僵尸。

当一只僵尸被判定死亡（也就是生命值归零，或被灰烬植物杀死等）前此回调函数立刻触发。

函数内此僵尸仍然被视为有效，自定数据将在所有此回调函数结束后被清理.
