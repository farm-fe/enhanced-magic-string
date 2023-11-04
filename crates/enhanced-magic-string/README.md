# enhanced-magic-string
Alternative Rust implementation of https://www.npmjs.com/package/magic-string with original sourcemap chain support.

This project is built for the [Farm](https://github.com/farm-fe/farm) project, which is a extremely fast vite-compatible web build tool written in Rust.

# RoadMap
Implement all APIs of [magic-string](https://www.npmjs.com/package/magic-string). `Bundle` will be implemented first.

## Bundle
- [x] addSource
- [x] generateMap
- [ ] append
- [ ] generateDecodedMap
- [ ] getIndentString
- [ ] indent
- [ ] prepend
- [x] toString
- [ ] isEmpty
- [ ] length
- [ ] trimLines
- [ ] trim
- [ ] trimStart
- [ ] trimEnd

## MagicString
- [ ] addSourcemapLocation
- [ ] append
- [ ] appendLeft
- [ ] appendRight
- [ ] clone
- [ ] generateDecodedMap
- [ ] generateMap
- [ ] getIndentString
- [ ] indent
- [ ] move
- [ ] overwrite
- [ ] update
- [ ] prepend
- [ ] prependLeft
- [ ] prependRight
- [ ] remove
- [ ] lastChar
- [ ] lastLine
- [ ] slice
- [ ] toString
- [ ] isEmpty
- [ ] length
- [ ] trimLines
- [ ] trim
- [ ] trimEndAborted
- [ ] trimEnd
- [ ] trimStartAborted
- [ ] trimStart
- [ ] hasChanged
- [ ] replace
- [ ] replaceAll