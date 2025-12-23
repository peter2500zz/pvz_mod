# 什么是 Rumia

Rumia 是一个为《植物大战僵尸》 v1.0.0.1051 版本提供 Lua mod 加载能力的插件。
这个词来自于 [露米娅](https://thwiki.cc/%E9%9C%B2%E7%B1%B3%E5%A8%85)，或者某种 Rust 和 Lua 的组合词。

Rumia 使用类似游戏《以撒的结合》的方式注册 Mod，并通过回调触发 Lua 函数，下面是一个简易的 Mod 示例：

```Lua
local mod = RegisterMod("My mod")

local function helloWorld()
    Log.info("Hello world!")
end

-- 注册回调，在加载完毕时调用 helloWorld 函数
mod:AddCallback(ModCallbacks.AT_GAME_INIT, helloWorld)
```

上面这个 Mod 几乎什么也没做，只是在游戏初始化完成时输出 `Hello world!` 到日志

Rumia 支持的回调点正在逐渐更新，且目前已经有许多好用的回调点。
