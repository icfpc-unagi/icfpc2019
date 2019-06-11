using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.IO;

namespace ICFPC2018
{
    class Program
    {
        static void Main(string[] args)
        {
            new Program().start(args);
        }

        bool[,,] block;
        int[,,] blockLayer;
        int[,,] blockReach;
        int[,,] blockDifficult;

        int[] LayerMaximumH;
        int[] LayerH;
        int[] LayerMove;

        //Lightning <-> Full
        int N, M = 40;  //LA:20 FULL:40
        bool UseGFill = false;

        int Layer = 1;
        int MaxLayer = 0;

        int[] LayerDiffcult;
        int[] LayerCnt;
        List<Tuple<int, int, int>>[] LayerList;


        long MINSCORE = -9999999999999999L;

        void start(string[] args)
        {
            if (args.Length < 1)
            {
                Console.Error.WriteLine("Error: FileName Required");
                return;
            }
            loadFile(args[0]);
            init();
            initMove();
            
            while(Layer <= MaxLayer)
            {
                //break;
                //Console.Error.WriteLine("NowLayer : " + Layer + " " + LayerCnt[Layer]);
                if(LayerH[Layer] == LayerMaximumH[Layer])
                {
                    setLayerEasy();
                }
                else
                {
                    setLayerHard();
                }
                Layer++;
            }
            foreach (var ss in ans)
            {
                Console.WriteLine(ss);
            }
            int cnt1 = 0;
            int cnt2 = 0;
            for (int x = 0; x < N; x++)
            {
                for (int y = 0; y < N; y++)
                {
                    for (int z = 0; z < N; z++)
                    {
                        if (block[x, y, z])
                        {
                            cnt1++;
                            if (blockState[x, y, z] == 4) cnt2++;
                            else
                            {
                                //Console.Error.WriteLine(blockLayer[x, y, z]);
                            }
                        }
                    }
                }
            }
            Console.Error.WriteLine("{0} / {1}", cnt2, cnt1);
        }

        int[] vy = { 1, 0, -1, 0 };
        int[] vx = { 0, 1, 0, -1 };

        void init()
        {
            N = loadByte(1);
            block = new bool[N, N, N];

            for (int x = 0; x < N; x++)
            {
                for (int z = 0; z < N; z++)
                {
                    for (int y = 0; y < N; y++)
                    {
                        if (loadBit(1) == 1)
                        {
                            block[x, y, z] = true;
                        }
                    }
                }
            }

            blockLayer = new int[N, N, N];
            blockDifficult = new int[N, N, N];
            blockReach = new int[N, N, N];
            q = new Queue<Tuple<int, int, int>>();
            q2 = new Queue<Tuple<int, int, int>>();
            nq = new Queue<Tuple<int, int, int>>();
            for (int x = 0; x < N; x++)
            {
                for (int y = 0; y < N; y++)
                {
                    if (block[x, y, 0])
                    {
                        blockLayer[x, y, 0] = 1;
                        q.Enqueue(Tuple.Create(x, y, 0));
                        q2.Enqueue(Tuple.Create(x, y, 0));
                    }
                }
            }
            ListUnderBlock();

            q = new Queue<Tuple<int, int, int>>();

            q.Enqueue(Tuple.Create(0, 0, 0));
            blockReach[0, 0, 0] = 9999;

            for (int x = 0; x < N; x++)
            {
                for (int y = 0; y < N; y++)
                {
                    for (int z = 0; z < N; z++)
                    {
                        if (!block[x, y, z])
                        {
                            //q.Enqueue(Tuple.Create(x, y, z));
                            //blockReach[x, y, z] = 9999;
                            blockLayer[x, y, z] = 9999;
                        }
                    }
                }
            }

            while (q.Count != 0)
            {
                var now = q.Dequeue();
                int x = now.Item1;
                int y = now.Item2;
                int z = now.Item3;

                ReachBlock(x + 1, y, z, blockReach[x, y, z]);
                ReachBlock(x - 1, y, z, blockReach[x, y, z]);
                ReachBlock(x, y + 1, z, blockReach[x, y, z]);
                ReachBlock(x, y - 1, z, blockReach[x, y, z]);
                ReachBlock(x, y, z + 1, blockReach[x, y, z]);
                ReachBlock(x, y, z - 1, blockReach[x, y, z]);
            }

            //bool ok = true;
            int cnt = 0;
            int ngcnt = 0;
            int ng2cnt = 0;
            int ng3cnt = 0;
            int ng4cnt = 0;

            for (int x = 0; x < N; x++)
            {
                for (int y = 0; y < N; y++)
                {
                    for (int z = 0; z < N; z++)
                    {
                        if (block[x, y, z])
                        {
                            cnt++;

                            if (LayerH[blockLayer[x, y, z]] == LayerMaximumH[blockLayer[x, y, z]]) continue;

                            blockDifficult[x, y, z] = 1;
                            ngcnt++;
                            bool ng = true;
                            for (int vz = -1; vz <= 1; vz++)
                            {
                                int nowz = z + vz;
                                if (!inside3(x, y, nowz)) continue;

                                for (int k = 0; k < 4; k++)
                                {
                                    int nowx = x;
                                    int nowy = y;
                                    while (inside(nowx, nowy))
                                    {
                                        if (block[nowx, nowy, nowz] && blockLayer[nowx, nowy, nowz] <= blockLayer[x, y, z]) break;
                                        nowx += vx[k];
                                        nowy += vy[k];
                                    }
                                    if (!inside(nowx, nowy))
                                    {
                                        ng = false;
                                        break;
                                    }
                                }
                            }
                            if (!ng)
                            {

                                continue;
                            }
                            ng2cnt++;

                            blockDifficult[x, y, z] = 2;

                            //Console.Error.WriteLine(blockReach[x, y, z] + " " + blockLayer[x, y, z]);
                            if (blockReach[x, y, z] == blockLayer[x, y, z]) continue;

                            blockDifficult[x, y, z] = 3;
                            ng3cnt++;

                            for (int vx = -1; vx <= 1; vx++)
                            {
                                for (int vy = -1; vy <= 1; vy++)
                                {
                                    for (int vz = -1; vz <= 1; vz++)
                                    {
                                        if (vx * vy * vz != 0) continue;
                                        int X = x + vx;
                                        int Y = y + vy;
                                        int Z = z + vz;
                                        if (!inside3(X, Y, Z)) continue;
                                        if (ng && blockReach[X, Y, Z] > blockLayer[x, y, z])
                                        {
                                            ng = false;
                                        }
                                    }
                                }
                            }

                            if (!ng)
                            {
                                continue;
                            }
                            blockDifficult[x, y, z] = 4;
                            ng4cnt++;
                        }
                    }
                }
            }
            Console.Error.WriteLine(ng4cnt + " / " + ng3cnt + " / " + ng2cnt + " / " + ngcnt + " / " + cnt);

            LayerDiffcult = new int[MaxLayer + 1];
            LayerCnt = new int[MaxLayer + 1];
            LayerList = new List<Tuple<int, int, int>>[MaxLayer + 1];

            for (int i = 0; i < MaxLayer + 1; i++)
            {
                LayerList[i] = new List<Tuple<int, int, int>>();
            }

            for (int x = 0; x < N; x++)
            {
                for (int y = 0; y < N; y++)
                {
                    for (int z = 0; z < N; z++)
                    {
                        if (block[x, y, z])
                        {
                            LayerDiffcult[blockLayer[x, y, z]] = Math.Max(LayerDiffcult[blockLayer[x, y, z]], blockDifficult[x, y, z]);
                            LayerCnt[blockLayer[x, y, z]]++;
                            LayerList[blockLayer[x, y, z]].Add(Tuple.Create(x, y, z));
                        }
                    }
                }
            }
        }

        Queue<Tuple<int, int, int>> q;
        Queue<Tuple<int, int, int>> q2;
        Queue<Tuple<int, int, int>> nq;

        void ListUnderBlock()
        {
            while (q.Count > 0)
            {
                while (q.Count > 0)
                {
                    var now = q.Dequeue();
                    setUnderBlock(now.Item1, now.Item2, now.Item3);
                }
                while (q2.Count > 0)
                {
                    var now = q2.Dequeue();
                    setUnderBlockUpDown(now.Item1, now.Item2, now.Item3);
                }
                while (nq.Count > 0)
                {
                    var now = nq.Dequeue();
                    q.Enqueue(now);
                    q2.Enqueue(now);
                }
            }

            Dictionary<int, int> dic = new Dictionary<int, int>();
            for (int x = 0; x < N; x++)
            {
                for (int y = 0; y < N; y++)
                {
                    for (int z = 0; z < N; z++)
                    {
                        if (block[x, y, z]) dic[blockLayer[x, y, z]] = 0;
                    }
                }
            }
            List<int> l = new List<int>();
            foreach (var num in dic.Keys)
            {
                l.Add(num);
            }
            l.Sort();

            MaxLayer = l.Count;
            LayerMaximumH = new int[MaxLayer + 1];
            LayerH = new int[MaxLayer + 1];
            LayerMove = new int[MaxLayer + 1];

            for (int i = 0; i < l.Count; i++)
            {
                dic[l[i]] = i + 1;
                LayerMaximumH[i + 1] = l[i] / 300000;
                LayerMove[i + 1] = (l[i] / 300) % 1000;
                LayerH[i + 1] = l[i] % 300;
                if (i == 0) LayerH[i + 1] = 0;
            }

            for (int x = 0; x < N; x++)
            {
                for (int y = 0; y < N; y++)
                {
                    for (int z = 0; z < N; z++)
                    {
                        if (block[x, y, z]) blockLayer[x, y, z] = dic[blockLayer[x, y, z]];
                    }
                }
            }
        }

        void setUnderBlock(int x, int y, int z)
        {
            setUnderBlock2(x + 1, y, z, blockLayer[x, y, z]);
            setUnderBlock2(x - 1, y, z, blockLayer[x, y, z]);
            setUnderBlock2(x, y + 1, z, blockLayer[x, y, z]);
            setUnderBlock2(x, y - 1, z, blockLayer[x, y, z]);
        }

        void setUnderBlockUpDown(int x, int y, int z)
        {
            int MaximumHeight = blockLayer[x, y, z] / 300000;
            int NowStep = (blockLayer[x, y, z] / 300) % 1000;

            setUnderBlock3(x, y, z + 1, encode(Math.Max(MaximumHeight, z + 1), NowStep + 1, z + 1));
            setUnderBlock3(x, y, z - 1, encode(Math.Max(MaximumHeight, z - 1), NowStep + 1, z - 1));
        }

        int encode(int a, int b, int c)
        {
            return a * 300000 + b * 300 + c;
        }

        void setUnderBlock2(int x, int y, int z, int cost)
        {
            if (!inside3(x, y, z)) return;
            if (!block[x, y, z]) return;
            if (blockLayer[x, y, z] == 0)
            {
                blockLayer[x, y, z] = cost;
                q.Enqueue(Tuple.Create(x, y, z));
                q2.Enqueue(Tuple.Create(x, y, z));
            }
        }

        void ReachBlock(int x, int y, int z, int cost)
        {
            if (!inside3(x, y, z)) return;
            if (blockReach[x, y, z] < Math.Min(cost - 1, blockLayer[x, y, z]))
            {
                blockReach[x, y, z] = Math.Min(cost, blockLayer[x, y, z]);
                q.Enqueue(Tuple.Create(x, y, z));
            }

        }

        void setUnderBlock3(int x, int y, int z, int cost)
        {
            if (!inside3(x, y, z)) return;
            if (!block[x, y, z]) return;
            if (blockLayer[x, y, z] == 0)
            {
                blockLayer[x, y, z] = cost;
                nq.Enqueue(Tuple.Create(x, y, z));
            }
        }

        int[,,] blockState;
        int[,,] blockTargetID;

        int[,,] TargetPoint;

        long ZLP = 100000;
        int P3 = 1000;
        int P2 = 5000;
        int P1 = 100;

        //put
        void SetBlock4(int x, int y, int z)
        {
            RemoveBlock(x, y, z);

            RemainTarget--;
            RemainUNFILL--;

            blockState[x, y, z] = 4;

            for (int dx = -1; dx <= 1; dx++)
            {
                int REM1 = 1 - Math.Abs(dx);
                for (int dy = -REM1; dy <= REM1; dy++)
                {
                    int REM2 = REM1 - Math.Abs(dy);
                    for (int dz = -REM2; dz <= REM2; dz++)
                    {
                        int X = x + dx;
                        int Y = y + dy;
                        int Z = z + dz;
                        if (!inside3(X, Y, Z)) continue;
                        if (blockState[X, Y, Z] == 1) SetBlock2(X, Y, Z);
                    }
                }
            }
        }


        //put(prepare)
        void SetBlock3(int x, int y, int z, bool flag = true)
        {
            RemoveBlock(x, y, z);

            if (flag && blockLayer[x, y, z] == Layer)
            {
                for (int dz = -1; dz <= 1; dz++)
                {
                    for (int dx = -1; dx <= 1; dx++)
                    {
                        for (int dy = -1; dy <= 1; dy++)
                        {
                            if (dz * dx * dy != 0) continue;
                            int X = x + dx;
                            int Y = y + dy;
                            int Z = z + dz;
                            if (!inside3(X, Y, Z)) continue;
                            TargetPoint[X, Y, Z] += P3;
                        }
                    }
                }
            }


            //RemainTarget--;
            blockState[x, y, z] = 3;
        }

        void RemoveBlock3(int x, int y, int z, bool flag = true)
        {
            //RemainTarget++;
            if (flag && blockLayer[x, y, z] == Layer)
            {
                for (int dz = -1; dz <= 1; dz++)
                {
                    for (int dx = -1; dx <= 1; dx++)
                    {
                        for (int dy = -1; dy <= 1; dy++)
                        {
                            if (dz * dx * dy != 0) continue;
                            int X = x + dx;
                            int Y = y + dy;
                            int Z = z + dz;
                            if (!inside3(X, Y, Z)) continue;
                            TargetPoint[X, Y, Z] -= P3;
                        }
                    }
                }
            }

            blockState[x, y, z] = 0;
        }

        //can put
        void SetBlock2(int x, int y, int z, bool flag = true)
        {
            RemoveBlock(x, y, z);
            if (flag && blockLayer[x, y, z] == Layer)
            {
                for (int dz = -1; dz <= 1; dz++)
                {
                    for (int dx = -1; dx <= 1; dx++)
                    {
                        for (int dy = -1; dy <= 1; dy++)
                        {
                            if (dz * dx * dy != 0) continue;
                            int X = x + dx;
                            int Y = y + dy;
                            int Z = z + dz;
                            if (!inside3(X, Y, Z)) continue;
                            TargetPoint[X, Y, Z] += P2;
                        }
                    }
                }
            }
            blockState[x, y, z] = 2;
        }

        void RemoveBlock2(int x, int y, int z, bool flag = true)
        {
            if (flag && blockLayer[x, y, z] == Layer)
            {
                for (int dz = -1; dz <= 1; dz++)
                {
                    for (int dx = -1; dx <= 1; dx++)
                    {
                        for (int dy = -1; dy <= 1; dy++)
                        {
                            if (dz * dx * dy != 0) continue;
                            int X = x + dx;
                            int Y = y + dy;
                            int Z = z + dz;
                            if (!inside3(X, Y, Z)) continue;
                            TargetPoint[X, Y, Z] -= P2;
                        }
                    }
                }
            }
            blockState[x, y, z] = 0;
        }

        //cannot put
        void SetBlock1(int x, int y, int z, bool flag = true)
        {
            RemoveBlock(x, y, z);

            if (flag && blockLayer[x, y, z] == Layer)
            {
                for (int dz = -1; dz <= 1; dz++)
                {
                    for (int dx = -1; dx <= 1; dx++)
                    {
                        for (int dy = -1; dy <= 1; dy++)
                        {
                            if (dz * dx * dy != 0) continue;
                            int X = x + dx;
                            int Y = y + dy;
                            int Z = z + dz;
                            if (!inside3(X, Y, Z)) continue;
                            TargetPoint[X, Y, Z] += P1;
                        }
                    }
                }
            }
            blockState[x, y, z] = 1;
        }

        void RemoveBlock1(int x, int y, int z, bool flag = true)
        {
            if (flag && blockLayer[x, y, z] == Layer)
            {
                for (int dz = -1; dz <= 1; dz++)
                {
                    for (int dx = -1; dx <= 1; dx++)
                    {
                        for (int dy = -1; dy <= 1; dy++)
                        {
                            if (dz * dx * dy != 0) continue;
                            int X = x + dx;
                            int Y = y + dy;
                            int Z = z + dz;
                            if (!inside3(X, Y, Z)) continue;
                            TargetPoint[X, Y, Z] -= P1;
                        }
                    }
                }
            }
            blockState[x, y, z] = 0;
        }

        void RemoveBlock(int x, int y, int z)
        {
            if (blockState[x, y, z] == 1) RemoveBlock1(x, y, z);
            if (blockState[x, y, z] == 2) RemoveBlock2(x, y, z);
            if (blockState[x, y, z] == 3) RemoveBlock3(x, y, z);
        }
        


        void initMove()
        {
            nextMove = new string[M];
            active = new bool[M];
            nextActive = new bool[M];
            seedID = new int[M];
            posx = new int[M];
            posy = new int[M];
            posz = new int[M];

            lastAccessTurn = new int[N, N, N];
            lastAccessID = new int[N, N, N];

            active[0] = true;
            ans = new List<string>();

            seedNum = new int[M];
            seedNum[0] = M - 1;

            dist = new int[N, N, N];
            

            blockState = new int[N, N, N];
            for (int x = 0; x < N; x++)
            {
                for (int y = 0; y < N; y++)
                {
                    for (int z = 1; z < N; z++)
                    {
                        if (block[x, y, z])
                        {
                            if (z == 0) blockState[x, y, z] = 2;
                            else blockState[x, y, z] = 1;
                        }
                    }
                }
            }


            blockTargetID = new int[N, N, N];
            TargetPoint = new int[N, N, N];


            SetTurnInit();

            for (int i = 0; i < N / 10; i++)
            {
                SetLMOVE(0, 0, 5, 1, 5);
                setAllAns();
            }
        }


        int RemainTarget;
        int RemainUNFILL;

        int MINZ, MAXZ = 0;

        int PreRemain = -1;
        int RemainLoop = 0;

        void setLayerEasy()
        {
            //if (Layer >= N) return;
            RemainTarget = 0;
            RemainUNFILL = 0;

            MINZ = MAXZ;

            foreach (var p in LayerList[Layer])
            {
                int X = p.Item1;
                int Y = p.Item2;
                int Z = p.Item3;
                
                RemainTarget++;
                RemainUNFILL++;

                MINZ = Math.Min(MINZ, Z + 1);
                MAXZ = Math.Max(MAXZ, Z + 1);

                if (blockState[X, Y, Z] == 1)
                {
                    blockState[X, Y, Z] = 0;
                    SetBlock1(X, Y, Z);
                }
                else
                {
                    blockState[X, Y, Z] = 0;
                    SetBlock2(X, Y, Z);
                }
            }

            MINZ = Math.Max(0, MINZ);
            MAXZ = Math.Min(N, MAXZ);
            int BMINZ = MINZ;

            for (int i = 0; i < M; i++)
            {
                if (blockReach[posx[i], posy[i], posz[i]] > Layer && posz[i] == MAXZ) SetTarget(i);
            }

            int TZ = LayerH[Layer];


            while (RemainUNFILL > 0)
            {
                if (PreRemain == RemainUNFILL + 9999 * Layer)
                {
                    RemainLoop++;
                    if (RemainLoop >= 30) throw new Exception(string.Format("LoopErrorEasy Turn: {0}  Layer: {1} Target: {2} UNFILL: {3} MAXZ : {4} NOWZ : {5}", turn, Layer, RemainTarget, RemainUNFILL, MAXZ, LayerH[Layer]));
                    if (RemainLoop >= 25) DebugView = true;
                }
                else
                {
                    PreRemain = RemainUNFILL + 9999 * Layer;
                    RemainLoop = 0;
                }

                MINZ = BMINZ;

                for (int i = 0; i < M; i++)
                {
                    if (!active[i]) continue;
                    if (posz[i] < BMINZ)
                    {
                        MINZ = Math.Max(0, Math.Min(MINZ, posz[i] - 3));
                    }
                }


                int pointCNT = 0;
                for (int i = 0; i < N; i++)
                {
                    for (int j = 0; j < N; j++)
                    {
                        if (TargetPoint[i, j, TZ + 1] > 0) pointCNT++;
                    }
                }



                Console.Error.WriteLine("Turn: {0}  Layer: {1} Target: {2} UNFILL: {3} POINTCNT: {4} Z:{5}/{6}/{7}", turn, Layer, RemainTarget, RemainUNFILL, pointCNT, MINZ, TZ, MAXZ);

                for (int i = 0; i < M; i++)
                {
                    if (!active[i]) continue;
                    if (DebugView || RemainLoop >= 25) Console.Error.WriteLine("NowID : " + i + " " + posx[i] + " " + posy[i] + " " + posz[i] + " " + TargetPoint[posx[i], posy[i], posz[i]]);
                    //Z-set
                    if (nextMove[i] == "" && posz[i] != MAXZ)
                    {
                        if (posz[i] < MAXZ - 1)
                        {
                            nextMove[i] = "ZFIT";
                            continue;
                        }
                        long bestPoint = MINSCORE;
                        int bestR1 = 0;
                        int bestD1 = 0;
                        int bestR2 = 0;
                        int bestD2 = 0;

                        for (int r1 = 0; r1 < 6; r1++)
                        {
                            int X = posx[i];
                            int Y = posy[i];
                            int Z = posz[i];
                            int add1;

                            add1 = 1;
                            if (r1 >= 3)
                            {
                                add1 = -1;
                            }
                            if (r1 == 5) continue;

                            for (int t = 1; t <= 15; t++)
                            {
                                if (r1 % 3 == 0) X += add1;
                                if (r1 % 3 == 1) Y += add1;
                                if (r1 % 3 == 2) Z += add1;

                                if (!inside3(X, Y, Z)) break;
                                if (lastAccessTurn[X, Y, Z] == turn) break;
                                if (blockState[X, Y, Z] == 4) break;

                                int X2 = X, Y2 = Y, Z2 = Z;
                                long Point;
                                if (!inside3(X2, Y2, Z2 - 1) || lastAccessTurn[X2, Y2, Z2 - 1] != turn || lastAccessID[X2, Y2, Z2 - 1] == i)
                                {
                                    X2 = X; Y2 = Y; Z2 = Z;
                                    Point = -Math.Abs(Z2 - MAXZ) * ZLP + TargetPoint[X2, Y2, Z2];
                                    if (Point > bestPoint)
                                    {
                                        bestPoint = Point;
                                        bestD1 = t;
                                        bestR1 = r1;
                                        bestD2 = -1;
                                        bestR2 = -1;
                                    }

                                    if (t <= 5)
                                    {
                                        for (int r2 = 0; r2 < 6; r2++)
                                        {
                                            X2 = X; Y2 = Y; Z2 = Z;
                                            if (r1 % 3 == r2 % 3) continue;
                                            if (r2 == 5) continue;
                                            int add2 = 1;
                                            add2 = 1;
                                            if (r2 >= 3)
                                            {
                                                add2 = -1;
                                            }

                                            for (int t2 = 1; t2 <= 5; t2++)
                                            {
                                                if (r2 % 3 == 0) X2 += add2;
                                                if (r2 % 3 == 1) Y2 += add2;
                                                if (r2 % 3 == 2) Z2 += add2;

                                                if (!inside3(X2, Y2, Z2)) break;
                                                if (lastAccessTurn[X2, Y2, Z2] == turn) break;
                                                //if (blockReach[X2, Y2, Z2] <= turn) break;
                                                if (blockState[X2, Y2, Z2] == 4) break;


                                                if (!inside3(X2, Y2, Z2 - 1) || lastAccessTurn[X2, Y2, Z2 - 1] != turn || lastAccessID[X2, Y2, Z2 - 1] == i)
                                                {
                                                    Point = -Math.Abs(Z2 - MAXZ) * ZLP + TargetPoint[X2, Y2, Z2];
                                                    if (Point > bestPoint)
                                                    {
                                                        bestPoint = Point;
                                                        bestD1 = t;
                                                        bestR1 = r1;
                                                        bestD2 = t2;
                                                        bestR2 = r2;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        if (bestPoint != MINSCORE)
                        {
                            if (DebugView) Console.Error.WriteLine("ID:{0} FitZ Point:{1}", i, bestPoint);
                            if (bestR2 == -1)
                            {
                                SetSMOVE(i, bestR1, bestD1);
                                if (blockReach[posx[i], posy[i], posz[i]] > Layer) SetTarget(i); ;

                            }
                            else
                            {
                                SetLMOVE(i, bestR1, bestD1, bestR2, bestD2);
                                if (blockReach[posx[i], posy[i], posz[i]] > Layer) SetTarget(i);
                            }
                        }
                    }

                    //GetChild
                    if (nextMove[i] == "" && seedNum[i] != 0)
                    {
                        for (int dx = 1; dx >= -1; dx--)
                        {
                            for (int dy = 1; dy >= -1 && nextMove[i] == ""; dy--)
                            {
                                int dz = 0;
                                int X = posx[i] + dx;
                                int Y = posy[i] + dy;
                                int Z = posz[i] + dz;

                                if (!inside3(X, Y, Z)) continue;
                                if (blockReach[X, Y, Z] <= Layer) continue;
                                if (lastAccessTurn[X, Y, Z] == turn) continue;


                                if (DebugView) Console.Error.WriteLine("ID:{0} MakeChild", i);
                                SetFISSION(i, dx, dy, dz, seedNum[i] / 2);
                            }
                        }
                    }
                }

                for (int i = 0; i < M; i++)
                {
                    if (!active[i]) continue;

                    //FillField
                    if (nextMove[i] == "" && blockReach[posx[i], posy[i], posz[i]] > Layer)
                    {
                        int dz = -1;
                        for (int dx = -1; dx <= 1 && nextMove[i] == ""; dx++)
                        {
                            for (int dy = -1; dy <= 1 && nextMove[i] == ""; dy++)
                            {
                                if (dx * dy * dz != 0) continue;
                                int X = posx[i] + dx;
                                int Y = posy[i] + dy;
                                int Z = posz[i] + dz;

                                if (!inside3(X, Y, Z)) continue;
                                if (lastAccessTurn[X, Y, Z] == turn) continue;

                                if ((blockState[X, Y, Z] == 2 || blockState[X,Y,Z] == 3) && blockLayer[X, Y, Z] == Layer)
                                {
                                    if (DebugView) Console.Error.WriteLine("ID:{0}{4} FILL  {1} {2} {3}", i, dx, dy, dz, ans.Count);
                                    SetFILL(i, dx, dy, dz);
                                }
                            }
                        }
                    }
                }

                for (int i = 0; i < M; i++)
                {
                    if (!active[i]) continue;
                    //1turn move
                    if (nextMove[i] == "")
                    {
                        long bestPoint = MINSCORE;
                        int bestR1 = 0;
                        int bestD1 = 0;
                        int bestR2 = 0;
                        int bestD2 = 0;

                        for (int r1 = 0; r1 < 6; r1++)
                        {
                            int X = posx[i];
                            int Y = posy[i];
                            int Z = posz[i];
                            int add1;

                            if (r1 % 3 == 2) continue;

                            add1 = 1;
                            if (r1 >= 3)
                            {
                                add1 = -1;
                            }

                            for (int t = 1; t <= 15; t++)
                            {
                                if (r1 % 3 == 0) X += add1;
                                if (r1 % 3 == 1) Y += add1;
                                if (r1 % 3 == 2) Z += add1;

                                if (!inside3(X, Y, Z)) break;
                                if (lastAccessTurn[X, Y, Z] == turn) break;
                                if (blockState[X, Y, Z] == 4) break;
                                //if (blockReach[X, Y, Z] <= Layer) break;

                                int X2 = X, Y2 = Y, Z2 = Z;
                                long Point;
                                if (!inside3(X2, Y2, Z2 - 1) || lastAccessTurn[X2, Y2, Z2 - 1] != turn || lastAccessID[X2, Y2, Z2 - 1] == i)
                                {
                                    if (TargetPoint[X2, Y2, Z2] >= P3)
                                    {
                                        X2 = X; Y2 = Y; Z2 = Z;
                                        Point = -Math.Abs(Z2 - MAXZ) * ZLP + TargetPoint[X2, Y2, Z2];
                                        Point *= 100; Point += t;

                                        if (Point > bestPoint)
                                        {
                                            bestPoint = Point;
                                            bestD1 = t;
                                            bestR1 = r1;
                                            bestD2 = -1;
                                            bestR2 = -1;
                                        }
                                    }

                                    if (t <= 5)
                                    {
                                        for (int r2 = 0; r2 < 6; r2++)
                                        {
                                            X2 = X; Y2 = Y; Z2 = Z;
                                            if (r1 % 3 == r2 % 3) continue;
                                            if (r2 % 3 == 2) continue;

                                            int add2 = 1;
                                            add2 = 1;
                                            if (r2 >= 3)
                                            {
                                                add2 = -1;
                                            }

                                            for (int t2 = 1; t2 <= 5; t2++)
                                            {
                                                if (r2 % 3 == 0) X2 += add2;
                                                if (r2 % 3 == 1) Y2 += add2;
                                                if (r2 % 3 == 2) Z2 += add2;

                                                if (!inside3(X2, Y2, Z2)) break;
                                                if (lastAccessTurn[X2, Y2, Z2] == turn) break;
                                                if (blockState[X2, Y2, Z2] == 4) break;
                                                //if (blockReach[X2, Y2, Z2] <= Layer) break;

                                                if (!inside3(X2, Y2, Z2 - 1) || lastAccessTurn[X2, Y2, Z2 - 1] != turn || lastAccessID[X2, Y2, Z2 - 1] == i)
                                                {
                                                    if (TargetPoint[X2, Y2, Z2] >= P3)
                                                    {
                                                        Point = -Math.Abs(Z2 - MAXZ) * ZLP + TargetPoint[X2, Y2, Z2];
                                                        Point *= 100; Point += t + t2;

                                                        if (Point > bestPoint)
                                                        {
                                                            bestPoint = Point;
                                                            bestD1 = t;
                                                            bestR1 = r1;
                                                            bestD2 = t2;
                                                            bestR2 = r2;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        if (bestPoint != MINSCORE)
                        {
                            if (DebugView) Console.Error.WriteLine("ID:{0} Move1 {1} {2} {3} {4} {5}", i, bestR1, bestD1, bestR2, bestD2, bestPoint);
                            if (bestR2 == -1)
                            {
                                SetSMOVE(i, bestR1, bestD1);
                                if (blockReach[posx[i], posy[i], posz[i]] > Layer) SetTarget(i);

                            }
                            else
                            {
                                SetLMOVE(i, bestR1, bestD1, bestR2, bestD2);
                                if (blockReach[posx[i], posy[i], posz[i]] > Layer) SetTarget(i);
                            }
                        }

                    }
                }

                for (int i = 0; i < M; i++)
                {
                    if (nextMove[i] == "ZFIT") nextMove[i] = "";
                }

                bool check = false;
                for (int i = 0; i < M; i++)
                {
                    if (nextMove[i] == "" && RemainTarget > 0) check = true;
                }
                if (RemainLoop >= 3) check = true;
                if (check) SetAllDist(true);

                bool c2 = false;
                if (MINZ != 0)
                {
                    for (int i = 0; i < M; i++)
                    {
                        if (dist[posx[i], posy[i], posz[i]] >= 9999 && posz[i] < TZ) c2 = true;
                    }
                }

                if (c2)
                {
                    MINZ = Math.Max(MINZ - 30, 0);
                    SetAllDist(true);
                }

                bool c3 = false;
                if (MINZ != 0)
                {
                    for (int i = 0; i < M; i++)
                    {
                        if (dist[posx[i], posy[i], posz[i]] >= 9999 && posz[i] < TZ - 2) c2 = true;
                    }
                }

                if (c3)
                {
                    MINZ = 0;
                    SetAllDist(true);
                }



                for (int i = 0; i < M; i++)
                {
                    if (!active[i]) continue;
                    //over2
                    if (nextMove[i] == "")
                    {
                        long bestPoint = MINSCORE;
                        int bestR1 = 0;
                        int bestD1 = 0;
                        int bestR2 = 0;
                        int bestD2 = 0;

                        for (int r1 = 0; r1 < 6; r1++)
                        {
                            int X = posx[i];
                            int Y = posy[i];
                            int Z = posz[i];
                            int add1;

                            if (r1 % 3 == 2) continue;

                            add1 = 1;
                            if (r1 >= 3)
                            {
                                add1 = -1;
                            }

                            for (int t = 1; t <= 15; t++)
                            {
                                if (r1 % 3 == 0) X += add1;
                                if (r1 % 3 == 1) Y += add1;
                                if (r1 % 3 == 2) Z += add1;

                                if (!inside3(X, Y, Z)) break;
                                if (lastAccessTurn[X, Y, Z] == turn) break;
                                if (Z > MAXZ) break;
                                if (Z < MINZ) break;

                                if (blockState[X, Y, Z] == 4) break;

                                //if (blockReach[X, Y, Z] <= turn) break;

                                int X2 = X, Y2 = Y, Z2 = Z;
                                long Point;
                                if (!inside3(X2, Y2, Z2 - 1) || lastAccessTurn[X2, Y2, Z2 - 1] != turn || lastAccessID[X2, Y2, Z2 - 1] == i)
                                {
                                    X2 = X; Y2 = Y; Z2 = Z;
                                    //Point = -Math.Abs(Z2 - MAXZ) * ZLP - dist[X2, Y2, Z2] * 100;
                                    Point = -dist[X2, Y2, Z2] * 100;

                                    if (Point > bestPoint)
                                    {
                                        bestPoint = Point;
                                        bestD1 = t;
                                        bestR1 = r1;
                                        bestD2 = -1;
                                        bestR2 = -1;
                                    }

                                    if (t <= 5)
                                    {
                                        for (int r2 = 0; r2 < 6; r2++)
                                        {
                                            X2 = X; Y2 = Y; Z2 = Z;
                                            if (r1 % 3 == r2 % 3) continue;
                                            if (r2 % 3 == 2) continue;

                                            int add2 = 1;
                                            add2 = 1;
                                            if (r2 >= 3)
                                            {
                                                add2 = -1;
                                            }

                                            for (int t2 = 1; t2 <= 5; t2++)
                                            {
                                                if (r2 % 3 == 0) X2 += add2;
                                                if (r2 % 3 == 1) Y2 += add2;
                                                if (r2 % 3 == 2) Z2 += add2;

                                                if (!inside3(X2, Y2, Z2)) break;
                                                if (lastAccessTurn[X2, Y2, Z2] == turn) break;

                                                if (blockState[X2, Y2, Z2] == 4) break;
                                                if (Z2 > MAXZ) break;
                                                if (Z2 < MINZ) break;

                                                //if (blockReach[X2, Y2, Z2] <= turn) break;

                                                if (!inside3(X2, Y2, Z2 - 1) || lastAccessTurn[X2, Y2, Z2 - 1] != turn || lastAccessID[X2, Y2, Z2 - 1] == i)
                                                {
                                                    //Point = -Math.Abs(Z2 - MAXZ) * ZLP - dist[X2, Y2, Z2] * 100;
                                                    Point = -dist[X2, Y2, Z2] * 100;

                                                    if (Point > bestPoint)
                                                    {
                                                        bestPoint = Point;
                                                        bestD1 = t;
                                                        bestR1 = r1;
                                                        bestD2 = t2;
                                                        bestR2 = r2;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        if (bestPoint != MINSCORE)
                        {
                            if (DebugView) Console.Error.WriteLine("ID:{0} Move2 {1} {2} {3} {4} {5}", i, bestR1, bestD1, bestR2, bestD2, bestPoint);
                            if (bestR2 == -1)
                            {
                                SetSMOVE(i, bestR1, bestD1);
                                if (blockReach[posx[i], posy[i], posz[i]] > Layer) SetTarget(i);

                            }
                            else
                            {
                                SetLMOVE(i, bestR1, bestD1, bestR2, bestD2);
                                if (blockReach[posx[i], posy[i], posz[i]] > Layer) SetTarget(i);
                            }
                        }
                    }

                }
                setAllAns();

            }
        }



        void setLayerHard()
        {
            Console.Error.WriteLine("Hard!");
            RemainTarget = 0;
            RemainUNFILL = 0;
            MINZ = MAXZ;

            foreach (var p in LayerList[Layer])
            {
                int X = p.Item1;
                int Y = p.Item2;
                int Z = p.Item3;
                
                RemainTarget++;
                RemainUNFILL++;

                MINZ = Math.Min(MINZ, Z - 1);
                MAXZ = Math.Max(MAXZ, Z + 1);

                if (blockState[X, Y, Z] == 1)
                {
                    blockState[X, Y, Z] = 0;
                    SetBlock1(X, Y, Z);
                }
                else
                {
                    blockState[X, Y, Z] = 0;
                    SetBlock2(X, Y, Z);
                }
            }

            MINZ = Math.Max(0, MINZ);
            MAXZ = Math.Min(N, MAXZ);
            int BMINZ = MINZ;
            int TZ = LayerH[Layer];

            for (int i = 0; i < M; i++)
            {
                if (blockReach[posx[i], posy[i], posz[i]] > Layer) SetTarget(i);
            }

            while (RemainUNFILL > 0)
            {
                if (PreRemain == RemainUNFILL)
                {
                    RemainLoop++;
                    if (RemainLoop >= 30) throw new Exception(string.Format("LoopErrorHard Turn: {0}  Layer: {1} Target: {2} UNFILL: {3} MAXZ : {4} NOWZ : {5}", turn, Layer, RemainTarget, RemainUNFILL, MAXZ, LayerH[Layer]));
                    if (RemainLoop >= 25) DebugView = true;
                }
                else
                {
                    PreRemain = RemainUNFILL;
                    RemainLoop = 0;
                }

                MINZ = BMINZ;

                for (int i = 0; i < M; i++)
                {
                    if (!active[i]) continue;
                    if (posz[i] < BMINZ)
                    {
                        MINZ = Math.Max(0, Math.Min(MINZ, posz[i] - 3));
                    }
                }


                int pointCNT = 0;
                for (int i = 0; i < N; i++)
                {
                    for (int j = 0; j < N; j++)
                    {
                        for (int k = MINZ; k <= MAXZ; k++)
                        {
                            if (TargetPoint[i, j, k] > 0) pointCNT++;
                        }
                    }
                }


                Console.Error.WriteLine("Turn: {0}  Layer: {1} Target: {2} UNFILL: {3} POINTCNT: {4} Z:{5}/{6}/{7}", turn, Layer, RemainTarget, RemainUNFILL, pointCNT, MINZ, TZ, MAXZ);
                for (int i = 0; i < M; i++)
                {
                    if (!active[i]) continue;
                    if (DebugView || RemainLoop >= 25) Console.Error.WriteLine("NowID : " + i + " " + posx[i] + " " + posy[i] + " " + posz[i] + " " + TargetPoint[posx[i], posy[i], posz[i]]);
                    //Z-set
                    if (nextMove[i] == "" && blockReach[posx[i], posy[i], posz[i]] <= Layer)
                    {
                        long bestPoint = MINSCORE;
                        int bestR1 = 0;
                        int bestD1 = 0;
                        int bestR2 = 0;
                        int bestD2 = 0;

                        for (int r1 = 0; r1 < 6; r1++)
                        {
                            int X = posx[i];
                            int Y = posy[i];
                            int Z = posz[i];
                            int add1;

                            add1 = 1;
                            if (r1 >= 3)
                            {
                                add1 = -1;
                            }

                            for (int t = 1; t <= 15; t++)
                            {
                                if (r1 % 3 == 0) X += add1;
                                if (r1 % 3 == 1) Y += add1;
                                if (r1 % 3 == 2) Z += add1;

                                if (!inside3(X, Y, Z)) break;
                                if (lastAccessTurn[X, Y, Z] == turn) break;
                                if (blockState[X, Y, Z] == 4) break;
                                //if (blockReach[X, Y, Z] <= Layer) break;

                                int X2 = X, Y2 = Y, Z2 = Z;
                                long Point;
                                if (!inside3(X2, Y2, Z2 - 1) || lastAccessTurn[X2, Y2, Z2 - 1] != turn || lastAccessID[X2, Y2, Z2 - 1] == i)
                                {
                                    X2 = X; Y2 = Y; Z2 = Z;
                                    Point = -Math.Max(0, Math.Abs(TZ - Z2) - 1) * ZLP + TargetPoint[X2, Y2, Z2];
                                    if (blockReach[X2, Y2, Z2] <= Layer) Point -= ZLP * 300;

                                    if (Point > bestPoint)
                                    {
                                        bestPoint = Point;
                                        bestD1 = t;
                                        bestR1 = r1;
                                        bestD2 = -1;
                                        bestR2 = -1;
                                    }

                                    if (t <= 5)
                                    {
                                        for (int r2 = 0; r2 < 6; r2++)
                                        {
                                            X2 = X; Y2 = Y; Z2 = Z;
                                            if (r1 % 3 == r2 % 3) continue;
                                            int add2 = 1;
                                            add2 = 1;
                                            if (r2 >= 3)
                                            {
                                                add2 = -1;
                                            }

                                            for (int t2 = 1; t2 <= 5; t2++)
                                            {
                                                if (r2 % 3 == 0) X2 += add2;
                                                if (r2 % 3 == 1) Y2 += add2;
                                                if (r2 % 3 == 2) Z2 += add2;

                                                if (!inside3(X2, Y2, Z2)) break;
                                                if (lastAccessTurn[X2, Y2, Z2] == turn) break;
                                                //if (blockReach[X2, Y2, Z2] <= Layer) break;
                                                if (blockState[X2, Y2, Z2] == 4) break;


                                                if (!inside3(X2, Y2, Z2 - 1) || lastAccessTurn[X2, Y2, Z2 - 1] != turn || lastAccessID[X2, Y2, Z2 - 1] == i)
                                                {
                                                    Point = -Math.Max(0, Math.Abs(TZ - Z2) - 1) * ZLP + TargetPoint[X2, Y2, Z2];
                                                    if (blockReach[X2, Y2, Z2] <= Layer) Point -= ZLP * 300;

                                                    if (Point > bestPoint)
                                                    {
                                                        bestPoint = Point;
                                                        bestD1 = t;
                                                        bestR1 = r1;
                                                        bestD2 = t2;
                                                        bestR2 = r2;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        if (bestPoint != MINSCORE)
                        {
                            if (DebugView) Console.Error.WriteLine("ID:{0} FitZ Point:{1}", i, bestPoint);
                            if (bestR2 == -1)
                            {
                                SetSMOVE(i, bestR1, bestD1);
                                if (blockReach[posx[i], posy[i], posz[i]] > Layer) SetTarget(i);

                            }
                            else
                            {
                                SetLMOVE(i, bestR1, bestD1, bestR2, bestD2);
                                if (blockReach[posx[i], posy[i], posz[i]] > Layer) SetTarget(i);
                            }
                        }
                    }

                    //GetChild
                    if (nextMove[i] == "" && seedNum[i] != 0)
                    {
                        for (int dx = 1; dx >= -1; dx--)
                        {
                            for (int dy = 1; dy >= -1 && nextMove[i] == ""; dy--)
                            {
                                int dz = 0;
                                int X = posx[i] + dx;
                                int Y = posy[i] + dy;
                                int Z = posz[i] + dz;

                                if (!inside3(X, Y, Z)) continue;
                                //
                                if (blockReach[X, Y, Z] <= Layer) continue;
                                if (lastAccessTurn[X, Y, Z] == turn) continue;
                                if (blockState[X, Y, Z] == 4) continue;


                                if (DebugView) Console.Error.WriteLine("ID:{0} MakeChild", i);
                                SetFISSION(i, dx, dy, dz, seedNum[i] / 2);
                            }
                        }
                    }
                }

                for (int i = 0; i < M; i++)
                {
                    if (!active[i]) continue;

                    //FillField
                    if (nextMove[i] == "" && blockReach[posx[i], posy[i], posz[i]] > Layer)
                    {
                        for (int dz = -1; dz <= 1 && nextMove[i] == ""; dz++)
                        {
                            for (int dx = -1; dx <= 1 && nextMove[i] == ""; dx++)
                            {
                                for (int dy = -1; dy <= 1 && nextMove[i] == ""; dy++)
                                {
                                    if (dx * dy * dz != 0) continue;
                                    int X = posx[i] + dx;
                                    int Y = posy[i] + dy;
                                    int Z = posz[i] + dz;

                                    if (!inside3(X, Y, Z)) continue;
                                    if (lastAccessTurn[X, Y, Z] == turn) continue;

                                    if (RemainLoop >= 25)
                                    {
                                        //Console.Error.WriteLine("PlaceCheck {0} {1} {2} = {3}", X, Y, Z, blockState[X, Y, Z]);
                                    }

                                    if ((blockState[X, Y, Z] == 2 || blockState[X, Y, Z] == 3) && blockLayer[X, Y, Z] == Layer)
                                    {
                                        if (DebugView) Console.Error.WriteLine("ID:{0}{4} FILL  {1} {2} {3}", i, dx, dy, dz, ans.Count);
                                        SetFILL(i, dx, dy, dz);
                                    }
                                }
                            }
                        }
                    }
                }

                for (int i = 0; i < M; i++)
                {
                    if (!active[i]) continue;
                    //1turn move

                    long M2 = MINSCORE;
                    if (nextMove[i] == "")
                    {
                        long bestPoint = M2;
                        int bestR1 = 0;
                        int bestD1 = 0;
                        int bestR2 = 0;
                        int bestD2 = 0;

                        for (int r1 = 0; r1 < 6; r1++)
                        {
                            int X = posx[i];
                            int Y = posy[i];
                            int Z = posz[i];
                            int add1;


                            add1 = 1;
                            if (r1 >= 3)
                            {
                                add1 = -1;
                            }

                            for (int t = 1; t <= 15; t++)
                            {
                                if (r1 % 3 == 0) X += add1;
                                if (r1 % 3 == 1) Y += add1;
                                if (r1 % 3 == 2) Z += add1;

                                if (!inside3(X, Y, Z)) break;
                                if (lastAccessTurn[X, Y, Z] == turn) break;
                                if (blockState[X, Y, Z] == 4) break;
                                //if (blockReach[X, Y, Z] <= Layer) break;

                                int X2 = X, Y2 = Y, Z2 = Z;
                                long Point;
                                if (!inside3(X2, Y2, Z2 - 1) || lastAccessTurn[X2, Y2, Z2 - 1] != turn || lastAccessID[X2, Y2, Z2 - 1] == i)
                                {
                                    if (TargetPoint[X2, Y2, Z2] >= P3)
                                    {
                                        X2 = X; Y2 = Y; Z2 = Z;
                                        Point = -Math.Max(0, Math.Abs(TZ - Z2) - 1) * ZLP + TargetPoint[X2, Y2, Z2];
                                        Point *= 100; Point += t;
                                        if (blockReach[X2, Y2, Z2] <= Layer) Point -= ZLP * 300;

                                        if (Point > bestPoint)
                                        {
                                            bestPoint = Point;
                                            bestD1 = t;
                                            bestR1 = r1;
                                            bestD2 = -1;
                                            bestR2 = -1;
                                        }
                                    }

                                    if (t <= 5)
                                    {
                                        for (int r2 = 0; r2 < 6; r2++)
                                        {
                                            X2 = X; Y2 = Y; Z2 = Z;
                                            if (r1 % 3 == r2 % 3) continue;

                                            int add2 = 1;
                                            add2 = 1;
                                            if (r2 >= 3)
                                            {
                                                add2 = -1;
                                            }

                                            for (int t2 = 1; t2 <= 5; t2++)
                                            {
                                                if (r2 % 3 == 0) X2 += add2;
                                                if (r2 % 3 == 1) Y2 += add2;
                                                if (r2 % 3 == 2) Z2 += add2;

                                                if (!inside3(X2, Y2, Z2)) break;
                                                if (lastAccessTurn[X2, Y2, Z2] == turn) break;
                                                if (blockState[X2, Y2, Z2] == 4) break;
                                                //if (blockReach[X2, Y2, Z2] <= Layer) break;

                                                if (!inside3(X2, Y2, Z2 - 1) || lastAccessTurn[X2, Y2, Z2 - 1] != turn || lastAccessID[X2, Y2, Z2 - 1] == i)
                                                {
                                                    if (TargetPoint[X2, Y2, Z2] >= P3)
                                                    {
                                                        Point = -Math.Max(0, Math.Abs(TZ - Z2) - 1) * ZLP + TargetPoint[X2, Y2, Z2];
                                                        Point *= 100; Point += t + t2;
                                                        if (blockReach[X2, Y2, Z2] <= Layer) Point -= ZLP * 300;

                                                        if (Point > bestPoint)
                                                        {
                                                            bestPoint = Point;
                                                            bestD1 = t;
                                                            bestR1 = r1;
                                                            bestD2 = t2;
                                                            bestR2 = r2;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        if (bestPoint != M2)
                        {
                            if (DebugView) Console.Error.WriteLine("ID:{0} Move1 {1} {2} {3} {4} {5}", i, bestR1, bestD1, bestR2, bestD2, bestPoint);
                            if (bestR2 == -1)
                            {
                                SetSMOVE(i, bestR1, bestD1);
                                if (blockReach[posx[i], posy[i], posz[i]] > Layer) SetTarget(i);

                            }
                            else
                            {
                                SetLMOVE(i, bestR1, bestD1, bestR2, bestD2);
                                if (blockReach[posx[i], posy[i], posz[i]] > Layer) SetTarget(i);
                            }
                        }

                    }
                }
                for (int i = 0; i < M; i++)
                {
                    if (nextMove[i] == "ZFIT") nextMove[i] = "";
                }


                bool check = false;
                for (int i = 0; i < M; i++)
                {
                    if (nextMove[i] == "" && RemainTarget > 0) check = true;
                }
                if (RemainLoop >= 3) check = true;
                if (check) SetAllDist();

                bool c2 = false;
                if (MINZ != 0)
                {
                    for (int i = 0; i < M; i++)
                    {
                        if (dist[posx[i], posy[i], posz[i]] >= 9999 && posz[i] < TZ) c2 = true;
                    }
                }

                if (c2)
                {
                    MINZ = Math.Max(MINZ - 30, 0);
                    SetAllDist();
                }

                bool c3 = false;
                if (MINZ != 0)
                {
                    for (int i = 0; i < M; i++)
                    {
                        if (dist[posx[i], posy[i], posz[i]] >= 9999 && posz[i] < TZ - 2) c2 = true;
                    }
                }

                if (c3)
                {
                    MINZ = 0;
                    SetAllDist();
                }



                for (int i = 0; i < M; i++)
                {
                    if (!active[i]) continue;
                    //over2
                    if (nextMove[i] == "")
                    {
                        long bestPoint = MINSCORE;
                        int bestR1 = 0;
                        int bestD1 = 0;
                        int bestR2 = 0;
                        int bestD2 = 0;

                        for (int r1 = 0; r1 < 6; r1++)
                        {
                            int X = posx[i];
                            int Y = posy[i];
                            int Z = posz[i];
                            int add1;
                            

                            add1 = 1;
                            if (r1 >= 3)
                            {
                                add1 = -1;
                            }

                            for (int t = 1; t <= 15; t++)
                            {
                                if (r1 % 3 == 0) X += add1;
                                if (r1 % 3 == 1) Y += add1;
                                if (r1 % 3 == 2) Z += add1;

                                if (!inside3(X, Y, Z)) break;
                                if (lastAccessTurn[X, Y, Z] == turn) break;
                                if (Z > MAXZ) break;
                                if (Z < MINZ) break;

                                if (blockState[X, Y, Z] == 4) break;

                                //if (blockReach[X, Y, Z] <= Layer) break;

                                int X2 = X, Y2 = Y, Z2 = Z;
                                long Point;
                                if (!inside3(X2, Y2, Z2 - 1) || lastAccessTurn[X2, Y2, Z2 - 1] != turn || lastAccessID[X2, Y2, Z2 - 1] == i)
                                {
                                    X2 = X; Y2 = Y; Z2 = Z;


                                    //Point = -Math.Max(0, Math.Abs(TZ - Z2) - 1) * ZLP - dist[X2, Y2, Z2] * 100;
                                    //if (blockReach[X2, Y2, Z2] <= Layer) Point -= ZLP * 300;

                                    Point = -dist[X2, Y2, Z2] * 100;
                                    if (blockReach[X2, Y2, Z2] <= Layer) Point -= ZLP * 300;

                                    if (Point > bestPoint)
                                    {
                                        bestPoint = Point;
                                        bestD1 = t;
                                        bestR1 = r1;
                                        bestD2 = -1;
                                        bestR2 = -1;
                                    }

                                    if (t <= 5)
                                    {
                                        for (int r2 = 0; r2 < 6; r2++)
                                        {
                                            X2 = X; Y2 = Y; Z2 = Z;
                                            if (r1 % 3 == r2 % 3) continue;

                                            int add2 = 1;
                                            add2 = 1;
                                            if (r2 >= 3)
                                            {
                                                add2 = -1;
                                            }

                                            for (int t2 = 1; t2 <= 5; t2++)
                                            {
                                                if (r2 % 3 == 0) X2 += add2;
                                                if (r2 % 3 == 1) Y2 += add2;
                                                if (r2 % 3 == 2) Z2 += add2;

                                                if (!inside3(X2, Y2, Z2)) break;
                                                if (lastAccessTurn[X2, Y2, Z2] == turn) break;

                                                if (blockState[X2, Y2, Z2] == 4) break;
                                                if (Z2 > MAXZ) break;
                                                if (Z2 < MINZ) break;

                                                //if (blockReach[X2, Y2, Z2] <= Layer) break;

                                                if (!inside3(X2, Y2, Z2 - 1) || lastAccessTurn[X2, Y2, Z2 - 1] != turn || lastAccessID[X2, Y2, Z2 - 1] == i)
                                                {
                                                    //Point = -Math.Max(0, Math.Abs(TZ - Z2) - 1) * ZLP - dist[X2, Y2, Z2] * 100;

                                                    Point = -dist[X2, Y2, Z2] * 100;
                                                    if (blockReach[X2, Y2, Z2] <= Layer) Point -= ZLP * 300;

                                                    if (Point > bestPoint)
                                                    {
                                                        bestPoint = Point;
                                                        bestD1 = t;
                                                        bestR1 = r1;
                                                        bestD2 = t2;
                                                        bestR2 = r2;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        if (bestPoint != MINSCORE)
                        {
                            if (DebugView) Console.Error.WriteLine("ID:{0} Move2 {1} {2} {3} {4} {5}", i, bestR1, bestD1, bestR2, bestD2, bestPoint);
                            if (bestR2 == -1)
                            {
                                SetSMOVE(i, bestR1, bestD1);
                                if (blockReach[posx[i], posy[i], posz[i]] > Layer) SetTarget(i);

                            }
                            else
                            {
                                SetLMOVE(i, bestR1, bestD1, bestR2, bestD2);
                                if (blockReach[posx[i], posy[i], posz[i]] > Layer) SetTarget(i);
                            }
                        }
                    }

                }
                setAllAns();
            }
        }



        void SetTarget(int a)
        {
            //return;
            int x = posx[a]; int y = posy[a]; int z = posz[a];
            for (int dz = -1; dz <= 1; dz++)
            {
                for (int dx = -1; dx <= 1; dx++)
                {
                    for (int dy = -1; dy <= 1; dy++)
                    {
                        if (dz * dx * dy != 0) continue;
                        int X = x + dx;
                        int Y = y + dy;
                        int Z = z + dz;
                        if (!inside3(X, Y, Z)) continue;
                        if(blockState[X,Y,Z] == 2 && blockLayer[X,Y,Z] == Layer)
                        {
                            SetBlock3(X, Y, Z);
                            blockTargetID[X, Y, Z] = a;
                        }
                    }
                }
            }

        }


        List<string> ans;
        string[] nextMove;
        bool[] active;
        bool[] nextActive;
        int[] seedID;
        int[] seedNum;
        int[] posx, posy, posz;

        int[,,] lastAccessTurn;
        int[,,] lastAccessID;

        int turn = 1;

        bool DebugView = false;

        void SetTurnInit()
        {
            nextMove = new string[M];
            for (int i = 0; i < M; i++)
            {
                nextMove[i] = "";
            }
            turn++;
            for (int i = 0; i < M; i++)
            {
                if (active[i])
                {
                    lastAccessTurn[posx[i], posy[i], posz[i]] = turn;
                    lastAccessID[posx[i], posy[i], posz[i]] = i;
                }
            }
        }

        void setAllAns()
        {
            for (int i = 0; i < M; i++)
            {
                if (active[i])
                {
                    //if(DebugView) ans.Add(string.Format("#ID {0}", i));
                    if (nextMove[i] == null || nextMove[i] == "")
                    {
                        ans.Add("WAIT");
                        if(DebugView) Console.Error.WriteLine("{0} Wait", i);
                    }
                    else
                    {
                        ans.Add(nextMove[i]);
                        if (DebugView)  Console.Error.WriteLine("{0} {1}", i, nextMove[i]);
                    }
                }
                else if (nextActive[i])
                {
                    active[i] = true;
                    nextActive[i] = false;
                }
            }
            SetTurnInit();

             //if (DebugView) Console.ReadLine();
        }

        void SetHALT(int a)
        {
            //nothing
        }

        void SetWAIT(int a)
        {
            nextMove[a] = "WAIT";
        }

        void SetFLIP(int a)
        {
            //nothing
        }

        void SetSMOVE(int a, int r1, int d1)
        {
            if(r1 >= 3)
            {
                d1 = -d1;
                r1 -= 3;
            }

            nextMove[a] = string.Format("SMOVE {0} {1}", RtoS[r1], d1);
            int X = posx[a];
            int Y = posy[a];
            int Z = posz[a];
            int add;
            
            add= 1;
            if(d1 < 0)
            {
                add = -1;
                d1 = Math.Abs(d1);
            }
            for (int t = 0; t < d1; t++)
            {
                if (r1 == 0) X += add;
                if (r1 == 1) Y += add;
                if (r1 == 2) Z += add;
                lastAccessID[X, Y, Z] = a;
                lastAccessTurn[X, Y, Z] = turn;
            }

            posx[a] = X;
            posy[a] = Y;
            posz[a] = Z;
        }

        string RtoS = "xzy";

        void SetLMOVE(int a, int r1, int d1, int r2, int d2)
        {
            if(r1 >= 3)
            {
                r1 -= 3;
                d1 = -d1;
            }
            if(r2 >= 3)
            {
                r2 -= 3;
                d2 = -d2;
            }


            nextMove[a] = string.Format("LMOVE {0} {1} {2} {3}", RtoS[r1], d1, RtoS[r2], d2);
            int X = posx[a];
            int Y = posy[a];
            int Z = posz[a];
            int add;

            add = 1;
            if (d1 < 0)
            {
                add = -1;
                d1 = Math.Abs(d1);
            }
            for (int t = 0; t < d1; t++)
            {
                if (r1 == 0) X += add;
                if (r1 == 1) Y += add;
                if (r1 == 2) Z += add;
                lastAccessID[X, Y, Z] = a;
                lastAccessTurn[X, Y, Z] = turn;
            }

            add = 1;
            if (d2 < 0)
            {
                add = -1;
                d2 = Math.Abs(d2);
            }
            for (int t = 0; t < d2; t++)
            {
                if (r2 == 0) X += add;
                if (r2 == 1) Y += add;
                if (r2 == 2) Z += add;
                lastAccessID[X, Y, Z] = a;
                lastAccessTurn[X, Y, Z] = turn;
            }

            posx[a] = X;
            posy[a] = Y;
            posz[a] = Z;

        }

        //FUSIONP
        //FUSIONS

        int SetFISSION(int a, int dx, int dy, int dz, int m)
        {
            int nextID = -1;
            int FM = m;
            nextMove[a] = string.Format("FISSION {0} {1} {2} {3}", dx, dz, dy, m);
            for (int i = 0; i < M; i++)
            {
                if (i != a && seedID[i] == a)
                {
                    nextID = i;
                    seedID[i] = nextID;
                    break;
                }
            }
            for (int i = nextID + 1; i < M; i++)
            {
                if (i != a && seedID[i] == a)
                {
                    seedID[i] = nextID;
                    m--;
                    if (m == 0) break;
                }
            }
            

            seedNum[a] -= FM + 1;
            seedNum[nextID] = FM;

            nextActive[nextID] = true;
            posx[nextID] = posx[a] + dx;
            posy[nextID] = posy[a] + dy;
            posz[nextID] = posz[a] + dz;

            lastAccessTurn[posx[nextID], posy[nextID], posz[nextID]] = turn;
            lastAccessID[posx[nextID], posy[nextID], posz[nextID]] = nextID;

            return nextID;
        }

        void SetFILL(int a, int dx, int dy, int dz)
        {
            nextMove[a] = string.Format("FILL {0} {1} {2}", dx, dz, dy);

            int X = posx[a] + dx;
            int Y = posy[a] + dy;
            int Z = posz[a] + dz;

            SetBlock4(X, Y, Z);
        }



        bool inside3(int x, int y, int z)
        {
            return x >= 0 && y >= 0 && z >= 0 && x < N && y < N && z < N;
        }

        byte[] buf;
        int cnt;
        int cnt2;

        int[,,] dist;

        int MAXDIST = 999999;

        void SetAllDist(bool downNG = false)
        {
            Queue<Tuple<int, int, int>> qtp = new Queue<Tuple<int, int, int>>();
            //MINZ = 0;
            for (int x = 0; x < N; x++)
            {
                for (int y = 0; y < N; y++)
                {
                    for (int z = MINZ; z <= MAXZ; z++)
                    {
                        dist[x, y, z] = MAXDIST;
                        if (blockReach[x, y, z] > Layer && TargetPoint[x, y, z] >= P3)
                        {
                            if (!downNG || z == MAXZ)
                            {
                                dist[x, y, z] = 0;
                                qtp.Enqueue(Tuple.Create(x, y, z));
                            }
                        }
                    }
                }
            }

            while(qtp.Count !=0){
                var now = qtp.Dequeue();
                int x = now.Item1;
                int y = now.Item2;
                int z = now.Item3;
                for (int r = 0; r < 6; r++)
                {
                    int X = x;
                    int Y = y;
                    int Z = z;
                    int add = 1;
                    if (r >= 3) add = -1;
                    if (r % 3 == 0) X += add;
                    if (r % 3 == 1) Y += add;
                    if (r % 3 == 2) Z += add;
                    if (!inside(X, Y)) continue;
                    if (Z < MINZ || Z > MAXZ) continue;
                    if (blockState[X, Y, Z] == 4) continue;
                    if (lastAccessTurn[X, Y, Z] == turn) continue; 
                    if (dist[X, Y, Z] != MAXDIST) continue;
                    dist[X, Y, Z] = dist[x, y, z] + 1;
                    qtp.Enqueue(Tuple.Create(X, Y, Z));
                }
            }
        }

        void loadFile(string FileName)
        {
            FileStream fs = new FileStream(FileName, FileMode.Open, FileAccess.Read);

            int fileSize = (int)fs.Length;
            buf = new byte[fileSize]; // データ格納用配列

            int readSize; // Readメソッドで読み込んだバイト数
            int remain = fileSize; // 読み込むべき残りのバイト数
            int bufPos = 0; // データ格納用配列内の追加位置
            cnt = 0;

            while (remain > 0)
            {
                // 1024Bytesずつ読み込む
                readSize = fs.Read(buf, bufPos, Math.Min(fileSize, remain));

                bufPos += readSize;
                remain -= readSize;
            }
            fs.Dispose();
        }

        int loadByte(int a)
        {
            int ans = 0;
            for (int i = 0; i < a; i++)
            {
                ans <<= 4;
                ans += buf[cnt++];
            }
            return ans;
        }

        int loadBit(int a)
        {
            int ans = (int)(buf[cnt]) >> (cnt2);
            ans &= 1;

            if (cnt2 == 7)
            {
                cnt++;
                cnt2 = 0;
            }
            else cnt2++;
            return ans;
        }

        bool inside(int y, int x)
        {
            return y >= 0 && x >= 0 && y < N && x < N;
        }
    }


}
