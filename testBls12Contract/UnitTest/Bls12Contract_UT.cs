using Microsoft.VisualStudio.TestTools.UnitTesting;
using Neo.Compiler.CSharp.UnitTests.Utils;
using System.IO;
using Neo;
using Neo.VM;
using Neo.VM.Types;
using Neo.Network.P2P.Payloads;
using Neo.Persistence;
using System;
using Neo.SmartContract.Manifest;
using Neo.Wallets;
using Neo.IO;
using Neo.SmartContract;
using Neo.IO.Json;
using System.Linq;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Threading.Tasks;
using System.Diagnostics;
using System.Threading;


namespace UnitTest
{

    [TestClass]

    public class Test
    {

        private TestEngine _engine;
        private byte[] testHash;

        class DummyVerificable : IVerifiable
        {
            public Witness[] Witnesses { get; set; }

            public int Size => 0;

            public void Deserialize(BinaryReader reader) { }

            public void DeserializeUnsigned(BinaryReader reader) { }

            public UInt160[] GetScriptHashesForVerifying(DataCache snapshot)
            {
                return new UInt160[]
                {
                    UInt160.Parse("0xb312313659b5da91e7662b603bdaad6329cb557a")
                };
            }

            public void Serialize(BinaryWriter writer) { }

            public void SerializeUnsigned(BinaryWriter writer) { }
        }



        [DllImport("bls12381", CallingConvention = CallingConvention.Cdecl)]
        public static extern IntPtr gt_add(IntPtr gt1, IntPtr gt2);

        [DllImport("bls12381", CallingConvention = CallingConvention.Cdecl)]
        public static extern IntPtr g1_g2_pairing(IntPtr g1, IntPtr g2);
        [DllImport("bls12381", CallingConvention = CallingConvention.Cdecl)]
        public static extern IntPtr test_generator_pairing();



        [DllImport("bls12381", CallingConvention = CallingConvention.Cdecl)]
        public static extern IntPtr g1_add_test();

        [DllImport("bls12381", CallingConvention = CallingConvention.Cdecl)]
        public static extern IntPtr g1_mul_test();

        [DllImport("bls12381", CallingConvention = CallingConvention.Cdecl)]
        public static extern IntPtr g2_add_test();

        [DllImport("bls12381", CallingConvention = CallingConvention.Cdecl)]
        public static extern IntPtr g2_mul_test();

        [DllImport("bls12381", CallingConvention = CallingConvention.Cdecl)]
        public static extern IntPtr gt_add_test();

        [DllImport("bls12381", CallingConvention = CallingConvention.Cdecl)]
        public static extern IntPtr gt_mul_test();

        [DllImport("bls12381", CallingConvention = CallingConvention.Cdecl)]
        public static extern IntPtr gt_neg_mul();

        [DllImport("bls12381", CallingConvention = CallingConvention.Cdecl)]

        public static extern IntPtr g1_generator();


        [DllImport("bls12381", CallingConvention = CallingConvention.Cdecl)]
        public static extern IntPtr g2_generator();

        [DllImport("bls12381", CallingConvention = CallingConvention.Cdecl)]
        public static extern void g1_check(IntPtr rawPtr);

        public static byte[] toByteArray(IntPtr res, int len)
        {
            if (res == IntPtr.Zero) Console.WriteLine("res: wrong");
            byte[] buffer = new byte[len];
            Marshal.Copy(res, buffer, 0, len);
            return buffer;
        }

        [TestInitialize]
        public void Init()
        {
            UInt160 defaultSender = UInt160.Parse("0xb312313659b5da91e7662b603bdaad6329cb557a");
            _engine = new TestEngine(TriggerType.Application, new DummyVerificable(), snapshot: new TestDataCache(), persistingBlock: new Block()
            {
                Header = new Header()
                {
                    Index = 123,
                    Timestamp = 1234,
                    Witness = new Witness()
                    {
                        InvocationScript = System.Array.Empty<byte>(),
                        VerificationScript = System.Array.Empty<byte>()
                    },
                    NextConsensus = UInt160.Zero,
                    MerkleRoot = UInt256.Zero,
                    PrevHash = UInt256.Zero
                },

                Transactions = new Transaction[]
                   {
                     new Transaction()
                     {
                          Attributes = System.Array.Empty<TransactionAttribute>(),
                          Signers = new Signer[]{ new Signer() { Account = defaultSender } },
                          Witnesses = System.Array.Empty<Witness>(),
                          Script = System.Array.Empty<byte>()
                     }
                   }


            });
            string path = "../../../../testContract/";

            string[] files = Directory.GetFiles(path, "*.cs");

            _engine.AddEntryScript(files);
            testHash = _engine.Nef.Script.ToScriptHash().ToString().Substring(2).HexToBytes().Reverse().ToArray();
            _engine.Snapshot.ContractAdd(new ContractState()
            {
                Hash = _engine.Nef.Script.ToScriptHash(),
                Nef = _engine.Nef,
                Manifest = Neo.SmartContract.Manifest.ContractManifest.FromJson(_engine.Manifest)

            });
            _engine.Snapshot.Commit();


        }

        private static Transaction BuildTransaction(UInt160 sender, byte[] script)
        {
            Transaction tx = new()
            {
                Script = script,
                Nonce = (uint)new Random().Next(1000, 9999),
                Signers = new Signer[]
                {
                    new() { Account = sender, Scopes = WitnessScope.Global }
                },
                Attributes = System.Array.Empty<TransactionAttribute>()
            };
            return tx;
        }

        [TestMethod]
        public void test_contract()
        {

            var g1o = g1_generator();
            byte[] g1 = toByteArray(g1o, 96);

            IntPtr p_add_test = g1_add_test();
            byte[] p_add_test_result = toByteArray(p_add_test, 96);

            var stack = _engine.ExecuteTestCaseStandard("pointAdd",g1,g1);
            Assert.AreEqual(VMState.HALT, _engine.State);
            Assert.AreEqual(p_add_test_result, stack.Pop());
            _engine.Reset();

            var g2o = g2_generator();
            byte[] g2 = toByteArray(g2o, 192);

            p_add_test = g2_add_test();
            p_add_test_result = toByteArray(p_add_test, 192);

            stack = _engine.ExecuteTestCaseStandard("pointAdd", g2, g2);
            Assert.AreEqual(VMState.HALT, _engine.State);
            Assert.AreEqual(p_add_test_result, stack.Pop());
            _engine.Reset();


            g1o = g1_generator();
            g1 = toByteArray(g1o, 96);

            IntPtr p_mul_test = g1_mul_test();
            byte[] p_mul_test_result = toByteArray(p_mul_test, 96);

            stack = _engine.ExecuteTestCaseStandard("pointMul", g1, 3);
            Assert.AreEqual(VMState.HALT, _engine.State);
            Assert.AreEqual(p_mul_test_result, stack.Pop());
            _engine.Reset();

            g2o = g2_generator();
            g2 = toByteArray(g2o, 192);

             p_mul_test = g2_mul_test();
            p_mul_test_result = toByteArray(p_mul_test, 192);

            stack = _engine.ExecuteTestCaseStandard("pointMul", g2, 3);
            Assert.AreEqual(VMState.HALT, _engine.State);
            Assert.AreEqual(p_mul_test_result, stack.Pop());
            _engine.Reset();


            stack = _engine.ExecuteTestCaseStandard("getOne");
            Assert.AreEqual(VMState.HALT, _engine.State);
            Assert.AreEqual(1, stack.Pop());


            _engine.Reset();

            g1o = g1_generator();
            g1 = toByteArray(g1o, 96);
            g2o = g2_generator();
            g2 = toByteArray(g2o, 192);
            IntPtr p_paring_test = test_generator_pairing();
            byte[] p_paring_test_result = toByteArray(p_paring_test, 576);
            stack = _engine.ExecuteTestCaseStandard("pointParing", g1, g2);
            Assert.AreEqual(VMState.HALT, _engine.State);
            Assert.AreEqual(p_paring_test_result, stack.Pop());

            byte[] gt = p_paring_test_result;
            _engine.Reset();


            p_add_test = gt_add_test();
            p_add_test_result = toByteArray(p_add_test, 576);
            stack = _engine.ExecuteTestCaseStandard("pointAdd", gt, gt);
            Assert.AreEqual(VMState.HALT, _engine.State);
            Assert.AreEqual(p_add_test_result, stack.Pop());
            _engine.Reset();


            p_mul_test = gt_mul_test();
            p_mul_test_result = toByteArray(p_mul_test, 576);
            stack = _engine.ExecuteTestCaseStandard("pointMul", gt, 3);
            Assert.AreEqual(VMState.HALT, _engine.State);
            Assert.AreEqual(p_mul_test_result, stack.Pop());
            _engine.Reset();

            p_mul_test = gt_neg_mul();
            p_mul_test_result = toByteArray(p_mul_test, 576);
            stack = _engine.ExecuteTestCaseStandard("pointMul", gt, -3);
            Assert.AreEqual(VMState.HALT, _engine.State);
            Assert.AreEqual(p_mul_test_result, stack.Pop());
            _engine.Reset();



        }
    }
}
