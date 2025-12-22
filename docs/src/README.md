# 简介

## 什么是 Rumia

Rumia 是一个为《植物大战僵尸》 1.0.0.1051 版本提供 Lua mod 加载能力的插件。
这个词来自于 [露米娅](https://thwiki.cc/%E9%9C%B2%E7%B1%B3%E5%A8%85)，或者某种 Rust 和 Lua 的组合词。

Rumia 使用类似游戏《以撒的结合》的方式注册 Mod，并通过回调触发 Lua 函数，下面是一个给所有僵尸绘制碰撞箱的 Mod 示例：

```Lua
-- 注册 Mod
local mod = RegisterMod("kiyuu")

-- 设定框的颜色
local borderColor = Color.New(0, 255, 0, 255)
local fillColor = Color.New(0, 255, 0, 63)

---@param g Graphics
local function atDraw(g)
    local board = GetLawnApp():GetBoard()
    -- 判断是否在关卡内
    if not board then return end

    -- 设置在关卡层绘制
    g:SetLayer(RenderLayers.Board)

    -- 遍历僵尸
    for _, zombie in pairs(board:GetZombies()) do
        -- 获取碰撞箱
        local hitbox = zombie:GetHitbox()

        -- 画内部
        g:SetColor(fillColor)
        g:FillRect(hitbox)
        -- 画边框
        g:SetColor(borderColor)
        g:DrawRect(hitbox)
    end
end

-- 注册回调
mod:AddCallback(ModCallbacks.AT_DRAW, atDraw)
```

上面的示例展现了使用 Lua 脚本注册函数回调、获取关卡内数据以及渲染的能力。

Rumia 支持的回调点正在逐渐更新，且目前已经有许多好用的回调点。
