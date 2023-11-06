    //constexpr unsigned int L = 1 << 24, N = 1 << 20; // Use with options A, B
    constexpr unsigned int L = 1 << 18, N = 1 << 14;  // Use with option C
    //constexpr unsigned int L = 1 << 22, N = 1 << 18;  // Use with option C, takes longer
    //constexpr unsigned int L = 1 << 8, N = 1 << 3;

    //system_clock::time_point t0 = system_clock::now();
    unique_ptr<char[]> s(new char[L]);
    vector<const char*> vs(N);
    {
        minstd_rand rgen; // unsigned 32 bit integer
        using rand_t = minstd_rand::result_type;

        if(0) for (             // Option A
            char* p = s.get(),  // get the head value from the char array
            *end = p + L;       // set to (init. mem pos. + 2^18)
            p != end;           // pointer reaches the end of the array
            p += sizeof(rand_t) // shift p by the type size of rng'd bytes
        ) {
            const rand_t x = rgen();    // rng new bytes
            ::memcpy(p, &x, sizeof(x)); // memcpy(dest, src, size)
        }
        else if(0) for (unsigned int i = 0; i < L; ++i) {       // Option B
            s[i] = 'a' + (rgen() % ('z' - 'a' + 1));
        }
        else { // Option C
            ::memset(
                s.get(),         // start of char array
                'a',             // fill data: the char 'a'
                L*sizeof(char)); // End of char array
            // Swap _some_ (L>>10) of the characters for other letters
            for (unsigned int i = 0; i < L/1024; ++i) { // L/1024==(L>>10)
                s[rgen() % (L - 1)] = // L-1: Oveflow protection?
                  'a' + (rgen() % ('z' - 'a' + 1));
            }
        }
        s[L-1] = 0;
        for (unsigned int i = 0; i < N; ++i) {
            vs[i] = &s[rgen() % (L - 1)];
        }
        //cout << "s=" << s.get() << endl;
        //for (unsigned int i = 0; i < N; ++i) cout << "vs[" << i << "]=" << vs[i] << endl;
    }
    system_clock::time_point t1 = system_clock::now();
    //cout << "Prep time(L=" << L << ", N=" << N << "): " << duration_cast<milliseconds>(t1 - t0).count() << "ms" << endl;
