---@meta

---别名
    ---@alias CallbackFunction fun(...): ...


---函数
    ---将日志写入到终端中
    ---@class Log
    ---@field info fun(...: any): nil @输出信息级别日志
    ---@field warn fun(...: any): nil @输出警告级别日志
    ---@field error fun(...: any): nil @输出错误级别日志
    ---@field debug fun(...: any): nil @输出调试级别日志
    ---@field trace fun(...: any): nil @输出追踪级别日志
    Log = {}

    ---注册一个模组 
    ---@param name string @模组名称
    ---@return Mod @返回模组对象
    function RegisterMod(name) end

    ---获取游戏类
    ---@return LawnApp @返回游戏对象
    function GetLawnApp() end


---模组类定义
    ---@class Mod @模组
    ---@field name string @模组名称
    ---@field priority integer @模组优先级
    ---@field AddCallback fun(self: Mod, callback: integer, function: CallbackFunction): nil @添加回调函数

    ---@class ModCallbacks @回调点
    ---游戏关卡分类
    ---@field AT_BOARD_KEYDOWN integer @游戏关卡内按键按下, fun(keycode: integer): nil
    ---@field AT_NEW_COIN integer @游戏关卡添加掉落物，fun(args: NewCoinArgs): nil
    ---@field AT_NEW_ZOMBIE integer @游戏关卡生成僵尸，fun(args: NewZombieArgs): nil
    ---@field AT_ZOMBIE_INIT integer @僵尸初始化，fun(zombie: Zombie): nil
    ---@field AT_ZOMBIE_UPDATE integer @僵尸更新，fun(zombie: Zombie): nil
    ModCallbacks = {}

    ---@class Vec2 @二维向量
    ---@field x number
    ---@field y number

    ---@class Rect2 @矩形
    ---@field x number
    ---@field y number
    ---@field width number
    ---@field height number
    ---@field position Vec2
    ---@field size Vec2

---游戏类定义
    ---@class LawnApp @游戏
    ---方法
    ---@field GetWindowSize fun(self): Vec2 @获取窗口尺寸
    ---@field GetBoard fun(self): Board? @获取关卡
    ---@field GetWidgetManager fun(self): WidgetManager? @获取控件管理器

    ---@class WidgetManager @控件管理器
    ---@field GetMousePos fun(self): Vec2 @获取鼠标坐标

    ---@class Board @关卡
    ---字段
    ---@field sun integer @关卡内的阳光值
    ---方法
    ---@field SetSun fun(self, value: integer) @设置关卡的阳光值
    ---@field AddZombie fun(self, zombie_type: integer, row: integer, from_wave: integer): Zombie @生成一只新的僵尸
    ---@field AddCoin fun(self, x: integer, y: integer, coin_type: integer, coin_motion): Coin @生成一只新的僵尸

    ---@class Zombie @僵尸
    ---字段
    ---@field body_hp integer @僵尸身体的血量
    ---方法
    ---@field IsValid fun(self): boolean @这个僵尸是否在内存中有效
    ---@field GetPos fun(self): Vec2 @获取僵尸坐标
    ---@field SetPos fun(self, x: number, y: number) @设定僵尸坐标

    ---@class Coin @掉落物


---回调参数定义
    ---AT_NEW_COIN
    ---@class NewCoinArgs
    ---@field x integer @掉落物产生在x坐标
    ---@field y integer @到落物产生在y坐标
    ---@field coin_type integer @掉落物的类型
    ---@field coin_motion integer @掉落物的运动方式

    ---AT_NEW_ZOMBIE
    ---@class ArgsNewZombie
    ---@field row integer @僵尸所在的行
    ---@field zombie_type integer @僵尸的类型
    ---@field from_wave integer @僵尸来自的波次
