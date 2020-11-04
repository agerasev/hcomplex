rtest_(constructor) {
    comp a = c_new(0.0, 1.0);
    assert_eq_(a.x, Approx(0.0));
    assert_eq_(a.y, Approx(1.0));
}
rtest_(inversion) {
    for (int i = 0; i < TEST_ATTEMPTS; ++i) {
        comp a = crng->nonzero();
        assert_eq_(c_div(a, a), approx(C1));
    }
}
rtest_(square_root) {
    for (int i = 0; i < TEST_ATTEMPTS; ++i) {
        comp a = crng->normal();
        comp b = c_sqrt(a);
        assert_eq_(c_mul(b, b), approx(a));
    }
}
rtest_(power) {
    for (int i = 0; i < TEST_ATTEMPTS; ++i) {
        comp a = crng->normal();
        int n = int(floor(2 + 10*rng->uniform()));
        comp b = c_pow_r(a, 1.0/n);
        comp c = C1;
        for (int i = 0; i < n; ++i) {
            c = c_mul(c, b);
        }
        assert_eq_(c, approx(a));
    }
}
rtest_(norm) {
    assert_eq_(c_norm_l1(c_new(-1, 2)), approx(3));
    assert_eq_(length(c_new(3, -4)), approx(5));
}
