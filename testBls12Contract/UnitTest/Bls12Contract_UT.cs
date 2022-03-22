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
        [DllImport("bls12381", CallingConvention = CallingConvention.Cdecl)]
        public static extern IntPtr gt_add(IntPtr gt1, IntPtr gt2);


        [DllImport("bls12381", CallingConvention = CallingConvention.Cdecl)]
        public static extern IntPtr g1_generator();

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


        }

        [TestMethod]
        public void test_g1_generator()
        {
            IntPtr g1Object = g1_generator();

            byte[] g1_bytes = toByteArray(g1Object, 96);

            g1_check(g1Object);
        }
    }
}