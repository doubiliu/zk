using System.ComponentModel;
using System.Numerics;
using Neo;
using Neo.SmartContract.Framework;
using Neo.SmartContract.Framework.Attributes;
using Neo.SmartContract.Framework.Native;
using Neo.SmartContract.Framework.Services;




namespace testBls12Contract
{
    [DisplayName("MToken")]
    [ManifestExtra("Author", "Neo")]
    [ManifestExtra("Email", "dev@neo.org")]
    [ManifestExtra("Description", "This is MToken")]
    [ContractPermission("*", "*")]
    public class testBls12Contract: SmartContract
    {
        public static BigInteger getOne() => 1;
        public static ByteString pointAdd(ByteString gt1, ByteString gt2)
        {
            ByteString result = Crypto.Bls12381Add(gt1, gt2);
            return result;
        }

        public static ByteString pointMul(ByteString gt, long mul)
        {
            ByteString result = Crypto.Bls12381Mul(gt, mul);
            return result;
        }

        public static ByteString pointParing(ByteString g1, ByteString g2)
        {
            ByteString result = Crypto.Bls12381Pairing(g1, g2);
            return result;
        }

    }
}
