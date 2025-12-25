---@meta

---AT_NEW_COIN
---@class NewCoinArgs
---@field GetPos fun(self): Vec2 @获取掉落物坐标
---@field SetPos fun(self, pos: Vec2) @设定掉落物坐标
---@field GetCoinType fun(self): integer @获取掉落物类型
---@field SetCoinType fun(self, coin_type: CoinType) @设定掉落物类型
---@field GetCoinMotion fun(self): integer @获取掉落物运动方式
---@field SetCoinMotion fun(self, coin_motion: CoinMotion) @设定掉落物运动方式

---AT_NEW_ZOMBIE
---@class ArgsNewZombie
---@field row integer @僵尸所在的行
---@field zombie_type integer @僵尸的类型
---@field from_wave integer @僵尸来自的波次
