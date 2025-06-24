import { XPSystem } from "../src/utils/XPSystem";

describe("XPSystem", () => {
  it("returns correct rank", () => {
    expect(XPSystem.getRank(0)).toBe("Initiate");
    expect(XPSystem.getRank(150)).toBe("Apprentice");
    expect(XPSystem.getRank(350)).toBe("Adept");
    expect(XPSystem.getRank(700)).toBe("Architect");
    expect(XPSystem.getRank(1200)).toBe("Grandmaster");
  });
}); 