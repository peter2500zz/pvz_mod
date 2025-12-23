# RegisterMod

`RegisterMod` 是注册 Rumia 可识别 Mod 的核心函数，它的签名如下:

`fun(name: string): Mod`

参数:

- `name` string: Mod 的名称

返回值:

- 注册的[Mod](../class/Mod.md)类，后续回调需要使用这个类进行注册。
