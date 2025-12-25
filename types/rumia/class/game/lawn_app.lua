---@meta

---@class LawnApp @游戏
---字段
---@field tigger fun(flag: string) @调试保留
---方法
---@field GetWindowSize fun(self): Vec2 @获取窗口尺寸
---@field GetMousePos fun(self): Vec2 @获取鼠标坐标
---@field GetBoard fun(self): Board? @获取关卡，如果还未创建则为空
