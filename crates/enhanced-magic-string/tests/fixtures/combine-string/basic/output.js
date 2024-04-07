/* header */
import { dep, formatTargetDir } from './modules/dep';
import { DepNoInlineSources } from './modules/dep-no-inline-sources';
export function init() {
    const targetDir = formatTargetDir('targetDir');
    console.log(targetDir);
    const depNoInlineSources = new DepNoInlineSources();
    console.log(depNoInlineSources.getA());
    depNoInlineSources.setA(dep);
    console.log(depNoInlineSources.getA());
}
//# sourceMappingURL=input.js.map
/* module */export class DepNoInlineSources {
    constructor() {
        this.a = 'a';
        const cc = 'c';
        this.a = this.a + cc;
    }
    setA(a) {
        this.a = a;
    }
    getA() {
        return this.a;
    }
}
//# sourceMappingURL=dep-no-inline-sources.js.map/* end of module */
/* module */export const dep = "dep";
export function formatTargetDir(targetDir) {
    return targetDir?.trim().replace(/\/+$/g, '');
}
//# sourceMappingURL=dep.js.map/* end of module *///# sourceMappingURL=output.js.map