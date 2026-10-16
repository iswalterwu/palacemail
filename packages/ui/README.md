# @palacemail/ui

PalaceMail 的界面组件与设计令牌。目前还没有组件，只有 `tokens/` 下的四类设计令牌占位文件。

## 主题即一组令牌

组件不写死任何颜色、字号、间距或动效时长，只消费 CSS 变量形式的设计令牌（`--pm-*`）。
一个主题就是这些变量的一组取值：切换主题只是替换取值，不改任何组件。

- `tokens/color.css` 里 `:root` 给出默认明亮主题，`[data-theme="dark"]` 给出默认暗色主题；
  应用负责在 `<html>` 上设置 `data-theme`（跟随系统或由用户选择）。
- 将来的东方风格视觉作为新主题加入，同样只是新增一组令牌取值（见 docs/architecture.md 第 4 节第 9 条）。

## 令牌分类

| 文件 | 内容 | 前缀 |
|---|---|---|
| `tokens/color.css` | 背景与表面、文字、边框、强调色（朱砂）、语义色 | `--pm-color-*` |
| `tokens/typography.css` | 字体族（中西文混排）、字号、行高、字重、字距 | `--pm-font-*`、`--pm-text-*`、`--pm-leading-*`、`--pm-weight-*`、`--pm-tracking-*` |
| `tokens/spacing.css` | 4px 基准的间距刻度、圆角 | `--pm-space-*`、`--pm-radius-*` |
| `tokens/motion.css` | 时长、缓动曲线；尊重"减少动态效果"偏好 | `--pm-duration-*`、`--pm-ease-*` |

`tokens/index.css` 汇总以上四类，应用只需在入口引入它：

```ts
import "@palacemail/ui/tokens/index.css";
```

所有取值都是占位，待视觉设计确定后替换；替换取值不需要改动任何消费方。

## 规划

组件将放在 `src/` 下，遵循以下原则：

- 只消费令牌，不出现字面量颜色与尺寸；
- AI 结果以安静的建议呈现（分类标签、摘要行、建议回复），组件不自行拼装提示词；
- 不直接调用模型，所有数据经由桌面壳的类型化 IPC 获得。
