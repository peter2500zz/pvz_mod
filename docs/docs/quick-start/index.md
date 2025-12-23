
# 创建你的第一个 Mod

在本节中，将介绍如何创建一个简单的僵尸碰撞箱绘制 Mod，并在游戏里查看它的效果。

!!! info "关于配置文件"

    鉴于我们将进行 Mod 的开发与调试，你最好在[配置文件](../configures/index.md)中打开终端显示，并设置一个合理的日志级别。

    以下是一个用于开发的日志配置，将内容保存为 `conf.yml`，然后就可以在弹出的终端或者 `pvz.log` 文件中查看日志。

    ```yaml
    show_terminal: true

    log_level: Debug
    ```

如果你已经启动过带 Rumia 的《植物大战僵尸》，你会发现目录下多了一个 `mods` 文件夹，如果没有，你可以自己创建一个。

在 `mods` 文件夹中新建一个文件夹，这将作为你的 Mod 文件夹，你可以叫它 `my-mod`，或者你喜欢的名字。

### Mod 注册

Rumia 会加载并执行每个 Mod 文件夹中的 `main.lua` 文件。向 `main.lua` 中写入以下内容可以向 Rumia 注册一个 Mod。

```lua
local mod = RegisterMod("This is My Mod")

-- 你也可以在加载这个 Mod 的时候记录点东西
Log.info("Hello world!")
```

!!! note

    对于所有可用的函数与类定义，可以参考仓库根目录的 [types.lua](https://github.com/peter2500zz/rumia/blob/master/types.lua) 文件。类似 EmmyLua 的插件可以识别其中的语法，并在编写 Mod 时提供帮助。

    你也可以查看 [API](../api/index.md) 页面来检索所有可用的函数与类定义。

保存这个文件，然后启动 Rumia，你应该会看到这样的日志。

```log
<rumia::mods/INFO> 正在加载 Mod: my-mod
<rumia::mods::log/INFO> Hello world!
```

### 添加回调

Rumia 基本是事件驱动型，因此为了绘制碰撞箱，我们需要添加一个回调函数，同时标记在 [AT_DRAW](../api/callback/AT_DRAW.md) 事件发生时调用回调函数。

我们顺便定义一些之后在本 Mod 内使用的变量。

```lua
...

-- 碰撞箱的颜色
local borderColor = Color.New(0, 255, 0, 255)
local fillColor = Color.New(0, 255, 0, 63)


---@param g Graphics
local function atDraw(g)
    -- TODO
end

-- 在绘制时调用我们的绘制函数
mod:AddCallback(ModCallbacks.AT_DRAW, atDraw)
```

!!! note

    有些回调会向回调函数传递参数，这些参数大多与回调有关，比如僵尸初始化的回调会传入僵尸，且同一个 Mod 的同一个回调点只能注册一个回调函数。

    关于回调函数的参数与返回值，请参考 [types.lua](https://github.com/peter2500zz/rumia/blob/master/types.lua)，或者 [API](../api/index.md) 页面。

如果你在这个函数里加入 `Log.info("drawing...")`，那么很显然，控制台会被刷屏。如果你想要观察到有效日志，建议减少在高频回调中记录日志，或者调整你的日志显示级别。

### 绘制碰撞箱

为了与游戏对象交互，Rumia 提供了 [GetLawnApp()](../api/global/GetLawnApp.md ) 函数用于获取代表游戏本身的类。我们这次需要使用它的成员方法 `LawnApp:GetBoard()`

```lua
---@param g Graphics
local function atDraw(g)
    local app = GetLawnApp()
    local board = app:GetBoard()
    -- 如果不在关卡内，board 将为 nil，强行调用会抛出 Lua 错误
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
```

在渲染部分，Rumia 采取延迟绘制的策略，也就是说代码先告诉 Rumia 将要绘制什么，到了特定的时机（也就是特定 RenderLayer），Rumia 便会绘制此前在代码中请求绘制的内容。

### 检查是否起效

打开《植物大战僵尸》游戏，并且进入任意一个关卡，你应该会在所有僵尸身上看到一个绿色矩形，这就是那个僵尸的碰撞箱。

你可以对你的 Mod 进行进一步修改，比如绘制僵尸的攻击判定框，或者在血量低于一定值的时候治疗僵尸。通过这些回调方法，能够做到很多事。

## 下一步做什么？

👉[API](../api/index.md) 这个页面中记录了 Rumia 目前提供的所有 API，翻翻它，看看有什么有意思的函数。

或者你可以看看这个由 AI 攥写的僵尸拖拽 Mod，它允许你用左键拖动僵尸。

```lua
-- 注册模组
local mod = RegisterMod("僵尸拖动")

-- 变量定义：用于存储当前正在拖拽的僵尸和鼠标偏移量
local currentDragZombie = nil ---@type Zombie?
local dragOffset = nil ---@type Vec2?

-- 辅助函数：判断点是否在矩形内
-- API 提供的 Rect2 包含 x, y, width, height
---@param point Vec2
---@param rect Rect2
---@return boolean
local function IsPointInRect(point, rect)
    return point.x >= rect.x and
        point.x <= (rect.x + rect.width) and
        point.y >= rect.y and
        point.y <= (rect.y + rect.height)
end

-- 回调：鼠标按下 (用于选中僵尸)
mod:AddCallback(ModCallbacks.AT_BOARD_MOUSE_DOWN, function(mousecode, pos)
    -- 只响应左键点击
    if mousecode ~= MouseCodes.L_CLICK then
        return
    end

    local app = GetLawnApp()
    if not app then return end

    local board = app:GetBoard()
    if not board then return end

    -- 获取场上所有僵尸
    local zombies = board:GetZombies()

    -- 遍历僵尸，寻找被点中的那个
    -- 注意：通常建议倒序遍历或检查 Z 轴，这里简单遍历，优先匹配到的会被拖拽
    for _, zombie in pairs(zombies) do
        if zombie:IsValid() then
            -- 获取僵尸的命中判定框
            local hitbox = zombie:GetHitbox()

            -- 判断鼠标点击坐标是否在判定框内
            if IsPointInRect(pos, hitbox) then
                currentDragZombie = zombie

                -- 计算偏移量：僵尸坐标 - 鼠标坐标
                -- 这样拖拽时，僵尸不会瞬间跳动到鼠标中心，而是保持相对位置
                local zombiePos = zombie:GetPos()
                dragOffset = Vec2.New(zombiePos.x - pos.x, zombiePos.y - pos.y)

                -- Log.info("开始拖拽僵尸 ID: " .. tostring(id))
                break -- 找到一个就退出，避免一次拖动多个
            end
        end
    end
end)

-- 回调：游戏逻辑更新 (用于移动僵尸)
mod:AddCallback(ModCallbacks.AT_BOARD_UPDATE, function(delta)
    local app = GetLawnApp()
    -- 如果当前没有拖拽僵尸或者游戏无效，直接返回
    if not currentDragZombie or not app then
        return
    end

    -- 关键检查：僵尸是否还在内存中有效（可能在拖拽过程中被植物打死了）
    if not currentDragZombie:IsValid() then
        currentDragZombie = nil
        dragOffset = nil
        return
    end

    local board = app and app:GetBoard()

    -- 安全检查：如果鼠标实际上已经松开了（UI外松开等情况），强制停止拖拽
    if board and not board:MousePressing() then
        currentDragZombie = nil
        dragOffset = nil
        return
    end

    if not dragOffset then
        dragOffset = Vec2.Zero()
    end

    -- 获取当前鼠标位置
    -- 注意：Update 回调不带鼠标参数，需要从 WidgetManager 获取
    local widgetMgr = app:GetWidgetManager()
    local currentMousePos = widgetMgr:GetMousePos()

    -- 计算新坐标：当前鼠标位置 + 初始偏移量
    local newX = currentMousePos.x + dragOffset.x
    local newY = currentMousePos.y + dragOffset.y

    -- 设置僵尸新位置
    currentDragZombie:SetPos(Vec2.New(newX, newY))
end)

-- 回调：鼠标松开 (用于释放僵尸)
mod:AddCallback(ModCallbacks.AT_BOARD_MOUSE_UP, function(mousecode, pos)
    -- 如果松开的是左键，且当前正在拖拽
    if mousecode == MouseCodes.L_CLICK and currentDragZombie then
        -- Log.info("释放僵尸")
        currentDragZombie = nil
        dragOffset = nil
    end
end)

Log.info("僵尸拖动已加载")
Log.info("按住左键拖动僵尸")
```
