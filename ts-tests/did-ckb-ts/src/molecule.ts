import {
  mol,
  HexLike,
  hexFrom,
  Hex,
  NumLike,
  numFrom,
  Num,
} from "@ckb-ccc/core";

// table DidCkbDataV1 {
//     document: Bytes,
//     localId: StringOpt,
// }
export type DidCkbDataV1Like = {
  document: HexLike;
  localId?: HexLike | null;
};

@mol.codec(
  mol.table({
    document: mol.Bytes,
    localId: mol.BytesOpt,
  }),
)
export class DidCkbDataV1 extends mol.Entity.Base<
  DidCkbDataV1Like,
  DidCkbDataV1
>() {
  constructor(
    public document: Hex,
    public localId?: Hex,
  ) {
    super();
  }

  static from(data: DidCkbDataV1Like): DidCkbDataV1 {
    if (data instanceof DidCkbDataV1) {
      return data;
    }
    return new DidCkbDataV1(
      hexFrom(data.document),
      data.localId ? hexFrom(data.localId) : undefined,
    );
  }
}

// union DidCkbData {
//   DidCkbDataV1,
// }

export type DidCkbDataLike = {
  value: DidCkbDataV1Like;
};

@mol.codec(
  mol.union({
    DidCkbDataV1,
  }),
)
export class DidCkbData extends mol.Entity.Base<DidCkbDataLike, DidCkbData>() {
  constructor(
    public type: "DidCkbDataV1",
    public value: DidCkbDataV1,
  ) {
    super();
  }

  static from(data: DidCkbDataLike): DidCkbData {
    if (data instanceof DidCkbData) {
      return data;
    }
    return new DidCkbData("DidCkbDataV1", DidCkbDataV1.from(data.value));
  }
}

// table PlcAuthorization {
//     history: BytesVec,
//     sig: Bytes,
//     rotationKeyIndices: Uint8Vec,
// }
export type PlcAuthorizationLike = {
  history: HexLike[];
  sig: HexLike;
  rotationKeyIndices: NumLike[];
};

@mol.codec(
  mol.table({
    history: mol.BytesVec,
    sig: mol.Bytes,
    rotationKeyIndices: mol.Uint8Vec,
  }),
)
export class PlcAuthorization extends mol.Entity.Base<
  PlcAuthorizationLike,
  PlcAuthorization
>() {
  constructor(
    public history: Hex[],
    public sig: Hex,
    public rotationKeyIndices: Num[],
  ) {
    super();
  }

  static from(data: PlcAuthorizationLike): PlcAuthorization {
    if (data instanceof PlcAuthorization) {
      return data;
    }
    return new PlcAuthorization(
      data.history.map((h) => hexFrom(h)),
      hexFrom(data.sig),
      data.rotationKeyIndices.map((s) => numFrom(s)),
    );
  }
}

// table DidCkbWitness {
//   localIdAuthorization: PlcAuthorization,
// }

export type DidCkbWitnessLike = {
  localIdAuthorization: PlcAuthorizationLike;
};

@mol.codec(
  mol.table({
    localIdAuthorization: PlcAuthorization,
  }),
)
export class DidCkbWitness extends mol.Entity.Base<
  DidCkbWitnessLike,
  DidCkbWitness
>() {
  constructor(public localIdAuthorization: PlcAuthorization) {
    super();
  }

  static from(data: DidCkbWitnessLike): DidCkbWitness {
    if (data instanceof DidCkbWitness) {
      return data;
    }
    return new DidCkbWitness(PlcAuthorization.from(data.localIdAuthorization));
  }
}

// a test molecule definition to test `compatible` flag
export type TestWitnessLike = {
  localIdAuthorization: PlcAuthorizationLike;
  padding: number;
};

@mol.codec(
  mol.table({
    localIdAuthorization: PlcAuthorization,
    padding: mol.Uint32,
  }),
)
export class TestWitness extends mol.Entity.Base<
  TestWitnessLike,
  TestWitness
>() {
  constructor(
    public localIdAuthorization: PlcAuthorization,
    public padding: number,
  ) {
    super();
  }

  static from(data: TestWitnessLike): TestWitness {
    if (data instanceof TestWitness) {
      return data;
    }
    return new TestWitness(PlcAuthorization.from(data.localIdAuthorization), 0);
  }
}
